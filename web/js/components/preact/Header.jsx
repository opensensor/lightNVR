/**
 * LightNVR Web Interface Header Component
 * Preact component for the site header
 */

import { useState, useEffect, useCallback } from 'preact/hooks';
import {VERSION} from '../../version.js';
import { fetchJSON } from '../../query-client.js';
import { getSettings } from '../../utils/settings-utils.js';
import { showStatusMessage } from './ToastContainer.jsx';
import { EditUserModal } from './users/EditUserModal.jsx';
import { getAuthHeaders, isDemoMode, validateSession } from '../../utils/auth-utils.js';
import { forceNavigation } from '../../utils/navigation-utils.js';
import { useI18n } from '../../i18n.js';

const buildProfileFormData = (user = {}) => ({
  username: user.username || '',
  password: '',
  email: user.email || '',
  role: user.role_id ?? 1,
  is_active: user.is_active ?? true,
  password_change_locked: user.password_change_locked ?? false,
  allowed_tags: '',
  allowed_login_cidrs: '',
});

/**
 * Header component
 * @param {Object} props - Component props
 * @param {string} props.version - System version
 * @returns {JSX.Element} Header component
 */
export function Header({ version = VERSION }) {
  // Get active navigation from data attribute on header container
  const headerContainer = document.getElementById('header-container');
  const activeNav = headerContainer?.dataset?.activeNav || '';
  const [username, setUsername] = useState('');
  const [currentUser, setCurrentUser] = useState(null);
  const [profileFormData, setProfileFormData] = useState(() => buildProfileFormData());
  const [isProfileModalOpen, setIsProfileModalOpen] = useState(false);
  const [isSavingProfile, setIsSavingProfile] = useState(false);
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
  const [authEnabled, setAuthEnabled] = useState(true); // Default to true while loading
  const [demoMode, setDemoMode] = useState(false); // Demo mode state
  const [userRole, setUserRole] = useState(null); // null = still loading
  const { t, locale, localePreference, setLocalePreference, availableLocales, AUTO_LOCALE } = useI18n();

  const syncSessionState = useCallback((session) => {
    if (session.valid && session.role) {
      setUserRole(session.role);
    } else {
      setUserRole(session.auth_enabled === false ? 'admin' : 'viewer');
    }

    const isSessionDemoMode = session.demo_mode === true;
    setDemoMode(isSessionDemoMode);

    if (session.username) {
      setUsername(session.username);
    } else {
      setUsername('');
    }

    if (session.id) {
      const nextUser = {
        id: session.id,
        username: session.username || '',
        email: session.email || '',
        role: session.role,
        role_id: session.role_id,
        is_active: session.is_active,
        password_change_locked: session.password_change_locked,
      };
      setCurrentUser(nextUser);
      setProfileFormData(buildProfileFormData(nextUser));
    } else {
      setCurrentUser(null);
      setProfileFormData(buildProfileFormData());
    }
  }, []);

  // Get the current username, role, and check if auth is enabled
  useEffect(() => {
    validateSession()
      .then(syncSessionState)
      .catch(() => {
        setUserRole('viewer');
        if (isDemoMode()) {
          setDemoMode(true);
        } else {
          setUsername('');
        }
      });

    // Fetch settings to check if auth is enabled
    async function checkAuthEnabled() {
      try {
        const settings = await getSettings();
        console.log('Header: Fetched settings:', settings);
        console.log('Header: web_auth_enabled value:', settings.web_auth_enabled);
        const isAuthEnabled = settings.web_auth_enabled === true;
        console.log('Header: Setting authEnabled to:', isAuthEnabled);
        setAuthEnabled(isAuthEnabled);
      } catch (error) {
        console.error('Error fetching auth settings:', error);
        // Default to true on error to avoid hiding logout button unnecessarily
        setAuthEnabled(true);
      }
    }
    checkAuthEnabled();

    // Also check demo mode from global state (set during session validation)
    const checkDemoMode = () => {
      if (window._demoMode === true) {
        setDemoMode(true);
        if (!currentUser?.id) {
          setUsername('');
        }
      }
    };
    // Check initially and also set up a listener for changes
    checkDemoMode();
    // Check periodically in case demo mode was set after initial load
    const intervalId = setInterval(checkDemoMode, 1000);
    // Clean up after first successful detection
    setTimeout(() => clearInterval(intervalId), 5000);
    return () => clearInterval(intervalId);
  }, [currentUser?.id, syncSessionState]);

  const handleProfileInputChange = useCallback((e) => {
    const { name, value, type, checked } = e.target;
    setProfileFormData(prevData => ({
      ...prevData,
      [name]: type === 'checkbox' ? checked : value,
    }));
  }, []);

  const openProfileModal = useCallback(() => {
    if (!currentUser?.id) {
      return;
    }

    setProfileFormData(buildProfileFormData(currentUser));
    setIsProfileModalOpen(true);
    setMobileMenuOpen(false);
  }, [currentUser]);

  const closeProfileModal = useCallback(() => {
    setIsProfileModalOpen(false);
  }, []);

  const handleProfileSave = useCallback(async (e) => {
    if (e) {
      e.preventDefault();
    }

    if (!currentUser?.id || isSavingProfile) {
      return;
    }

    setIsSavingProfile(true);
    try {
      const updatedUser = await fetchJSON(`/api/auth/users/${currentUser.id}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          ...getAuthHeaders(),
        },
        body: JSON.stringify({
          username: profileFormData.username.trim(),
          email: profileFormData.email.trim(),
        }),
        timeout: 15000,
        retries: 1,
        retryDelay: 1000,
      });

      const nextUser = {
        id: updatedUser.id,
        username: updatedUser.username,
        email: updatedUser.email || '',
        role: currentUser.role,
        role_id: updatedUser.role,
        is_active: updatedUser.is_active,
        password_change_locked: updatedUser.password_change_locked,
      };

      setCurrentUser(nextUser);
      setProfileFormData(buildProfileFormData(nextUser));
      setUsername(updatedUser.username);
      setIsProfileModalOpen(false);
      showStatusMessage(t('auth.profileUpdated'), 'success', 5000);
    } catch (error) {
      console.error('Error updating current user:', error);
      showStatusMessage(t('auth.profileUpdateError', { message: error.message }), 'error', 8000);
    } finally {
      setIsSavingProfile(false);
    }
  }, [currentUser, isSavingProfile, profileFormData.email, profileFormData.username, t]);

  // Toggle mobile menu
  const toggleMobileMenu = () => {
    setMobileMenuOpen(!mobileMenuOpen);
  };

  // Special handling for Live View link to handle both index.html and root URL
  const getLiveViewHref = () => {
    // Check if we're on the root URL or index.html
    const isRoot = window.location.pathname === '/' || window.location.pathname.endsWith('/');

    // If we're on the root URL, stay on the root URL
    if (isRoot) {
      return './';
    }

    // Otherwise, default to index.html
    return 'index.html';
  };

  // Determine if the current user has admin access for nav filtering.
  // While the role is still loading (null) we conservatively show all items
  // so the nav doesn't flash/reorder after load.
  const isAdmin = userRole === null || userRole === 'admin';
  const canEditCurrentUser = authEnabled && !demoMode && Boolean(currentUser?.id);
  const displayUsername = username || (demoMode ? t('auth.demoViewer') : t('auth.user'));
  const activeLocale = availableLocales.find((item) => item.code === locale);
  const browserDefaultLabel = activeLocale
    ? `${t('language.browserDefault')} (${activeLocale.nativeName})`
    : t('language.browserDefault');

  // Navigation items - don't preserve query parameters when navigating via header
  // Admin-only tabs (System, Users) are hidden from non-admin roles.
  const navItems = [
    { id: 'nav-live', href: getLiveViewHref(), label: t('nav.live') },
    { id: 'nav-recordings', href: 'recordings.html', label: t('nav.recordings') },
    { id: 'nav-streams', href: 'streams.html', label: t('nav.streams') },
    { id: 'nav-settings', href: 'settings.html', label: t('nav.settings') },
    ...(isAdmin ? [{ id: 'nav-users', href: 'users.html', label: t('nav.users') }] : []),
    ...(isAdmin ? [{ id: 'nav-system', href: 'system.html', label: t('nav.system') }] : []),
  ];

  const handleLocaleSelection = useCallback(async (value) => {
    try {
      await setLocalePreference(value === AUTO_LOCALE ? null : value);
    } catch (error) {
      console.error('Error changing locale:', error);
      showStatusMessage(t('language.changeError'), 'error', 5000);
    }
  }, [AUTO_LOCALE, setLocalePreference, t]);

  // Render navigation item
  const renderNavItem = (item) => {
    const isActive = activeNav === item.id;
    const baseClasses = "no-underline rounded transition-colors";
    const desktopClasses = "px-3 py-2";
    const mobileClasses = "block w-full px-4 py-3 text-left";
    const activeClass = isActive ? 'bg-[hsl(var(--primary))] text-[hsl(var(--primary-foreground))]' : 'text-[hsl(var(--card-foreground))] hover:bg-[hsl(var(--primary)/0.8)] hover:text-[hsl(var(--primary-foreground))]';

    return (
        <li className={mobileMenuOpen ? "w-full" : "mx-1"}>
          <a
              href={item.href}
              id={item.id}
              className={`${baseClasses} ${mobileMenuOpen ? mobileClasses : desktopClasses} ${activeClass}`}
              onClick={(e) => {
                // Force navigation and prevent default behavior
                forceNavigation(item.href, e);

                // Close mobile menu if open
                if (mobileMenuOpen) {
                  toggleMobileMenu();
                }
              }}
          >
            {item.label}
          </a>
        </li>
    );
  };

  const renderUsername = (mobile = false) => {
    if (!canEditCurrentUser) {
      return <span>{displayUsername}</span>;
    }

    return (
      <button
        type="button"
        className={`bg-transparent border-0 p-0 font-medium transition-colors ${mobile ? 'text-left' : ''}`}
        style={{ color: 'hsl(var(--card-foreground))' }}
        onClick={openProfileModal}
        title={t('auth.editProfile')}
      >
        {displayUsername}
      </button>
    );
  };

  const renderLanguageSelector = (mobile = false) => (
    <div className={mobile ? 'w-full px-4 py-2' : 'mr-3'}>
      {mobile && (
        <div className="mb-1 text-xs uppercase tracking-wide text-muted-foreground">
          {t('language.label')}
        </div>
      )}
      <select
        aria-label={t('language.label')}
        className="rounded border border-input bg-background px-2 py-1 text-sm text-foreground"
        value={localePreference || AUTO_LOCALE}
        onChange={(e) => {
          void handleLocaleSelection(e.currentTarget.value);
        }}
      >
        <option value={AUTO_LOCALE}>{browserDefaultLabel}</option>
        {availableLocales.map((item) => (
          <option key={item.code} value={item.code}>{item.nativeName}</option>
        ))}
      </select>
    </div>
  );

  return (
      <>
      <header className="app-header py-2 shadow-md mb-4 w-full" style={{ position: 'relative', zIndex: 20, backgroundColor: 'hsl(var(--card))', color: 'hsl(var(--card-foreground))' }}>
        <div className="container mx-auto px-4 flex justify-between items-center">
          <div className="logo flex items-center">
            <h1 className="text-xl font-bold m-0">LightNVR</h1>
            <span className="version text-xs ml-2" style={{color: 'hsl(var(--muted-foreground))'}}>v{version}</span>
          </div>

          {/* Desktop Navigation */}
          <nav className="hidden md:block" style={{ position: 'relative', zIndex: 20 }}>
            <ul className="flex list-none m-0 p-0">
              {navItems.map(renderNavItem)}
            </ul>
          </nav>

          {/* User Menu (Desktop) */}
          <div className="user-menu hidden md:flex items-center">
            {renderLanguageSelector()}
            {demoMode && !localStorage.getItem('auth') && (
              <span className="mr-2 px-2 py-0.5 text-xs rounded" style={{backgroundColor: 'hsl(var(--accent))', color: 'hsl(var(--accent-foreground))'}}>{t('auth.demoMode')}</span>
            )}
            <div className="mr-2">{renderUsername()}</div>
            {authEnabled && (
              demoMode && !localStorage.getItem('auth') ? (
                <a
                  href="/login.html"
                  className="login-link no-underline px-3 py-1 rounded transition-colors"
                  style={{
                    color: 'hsl(var(--primary-foreground))',
                    backgroundColor: 'hsl(var(--primary))'
                  }}
                  onMouseOver={(e) => e.currentTarget.style.backgroundColor = 'hsl(var(--primary) / 0.8)'}
                  onMouseOut={(e) => e.currentTarget.style.backgroundColor = 'hsl(var(--primary))'}
                >
                  {t('auth.login')}
                </a>
              ) : (
                <a
                  href="/logout"
                  className="logout-link no-underline px-3 py-1 rounded transition-colors"
                  style={{
                    color: 'hsl(var(--card-foreground))',
                    backgroundColor: 'transparent'
                  }}
                  onMouseOver={(e) => e.currentTarget.style.backgroundColor = 'hsl(var(--primary) / 0.8)'}
                  onMouseOut={(e) => e.currentTarget.style.backgroundColor = 'transparent'}
                >
                  {t('auth.logout')}
                </a>
              )
            )}
          </div>

          {/* Mobile Menu Button */}
          <button
              className="md:hidden p-2 focus:outline-none"
              style={{color: 'hsl(var(--card-foreground))'}}
              onClick={toggleMobileMenu}
              aria-label={t('nav.toggleMenu')}
          >
            <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d={mobileMenuOpen ? "M6 18L18 6M6 6l12 12" : "M4 6h16M4 12h16M4 18h16"} />
            </svg>
          </button>
        </div>

        {/* Mobile Navigation */}
        {mobileMenuOpen && (
            <div className="md:hidden mt-2 border-t pt-2 container mx-auto px-4" style={{borderColor: 'hsl(var(--border))'}}>
              <ul className="list-none m-0 p-0 flex flex-col w-full">
                <li className="w-full">{renderLanguageSelector(true)}</li>
                {navItems.map(renderNavItem)}
                {authEnabled && (
                  <li className="w-full mt-2 pt-2 border-t" style={{borderColor: 'hsl(var(--border))'}}>
                    <div className="flex justify-between items-center px-4 py-2">
                      <div className="flex items-center">
                        {demoMode && !localStorage.getItem('auth') && (
                          <span className="mr-2 px-2 py-0.5 text-xs rounded" style={{backgroundColor: 'hsl(var(--accent))', color: 'hsl(var(--accent-foreground))'}}>{t('auth.demoShort')}</span>
                        )}
                        {renderUsername(true)}
                      </div>
                      {demoMode && !localStorage.getItem('auth') ? (
                        <a
                          href="/login.html"
                          className="login-link no-underline px-3 py-1 rounded transition-colors"
                          style={{
                            color: 'hsl(var(--primary-foreground))',
                            backgroundColor: 'hsl(var(--primary))'
                          }}
                          onMouseOver={(e) => e.currentTarget.style.backgroundColor = 'hsl(var(--primary) / 0.8)'}
                          onMouseOut={(e) => e.currentTarget.style.backgroundColor = 'hsl(var(--primary))'}
                        >
                          {t('auth.login')}
                        </a>
                      ) : (
                        <a
                          href="/logout"
                          className="logout-link no-underline px-3 py-1 rounded transition-colors"
                          style={{
                            color: 'hsl(var(--card-foreground))',
                            backgroundColor: 'transparent'
                          }}
                          onMouseOver={(e) => e.currentTarget.style.backgroundColor = 'hsl(var(--primary) / 0.8)'}
                          onMouseOut={(e) => e.currentTarget.style.backgroundColor = 'transparent'}
                        >
                          {t('auth.logout')}
                        </a>
                      )}
                    </div>
                  </li>
                )}
              </ul>
            </div>
        )}
      </header>
      {isProfileModalOpen && currentUser && (
        <EditUserModal
          currentUser={currentUser}
          formData={profileFormData}
          handleInputChange={handleProfileInputChange}
          handleEditUser={handleProfileSave}
          onClose={closeProfileModal}
          title={t('auth.editProfileTitle')}
          submitLabel={isSavingProfile ? t('common.saving') : t('common.saveChanges')}
          showPasswordField={false}
          showRoleField={false}
          showActiveField={false}
          showPasswordLockField={false}
          showAllowedTagsField={false}
          showAllowedLoginCidrsField={false}
          showClearLoginLockoutButton={false}
        />
      )}
      </>
  );
}