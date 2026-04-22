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
import LanguageSelector from './common/LanguageSelector.jsx';

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
  const [username, _setUsername] = useState(localStorage.getItem('username') || '');
  const [currentUser, setCurrentUser] = useState(null);
  const [profileFormData, setProfileFormData] = useState(() => buildProfileFormData());
  const [isProfileModalOpen, setIsProfileModalOpen] = useState(false);
  const [isSavingProfile, setIsSavingProfile] = useState(false);
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
  const [authEnabled, setAuthEnabled] = useState(true); // Default to true while loading
  const [demoMode, setDemoMode] = useState(false); // Demo mode state
  const [userRole, _setUserRole] = useState(localStorage.getItem('userrole') || null); // null = still loading
  const { t } = useI18n();

  const setUsername = (username) => {
    _setUsername(username);
    localStorage.setItem('username', username);
  };

  const setUserRole = (userrole) => {
    _setUserRole(userrole);
    localStorage.setItem('userrole', userrole);
  };

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
      const payload = {
        username: profileFormData.username.trim(),
        email: profileFormData.email.trim(),
      };
      if (profileFormData.password) {
        payload.password = profileFormData.password;
      }
      const updatedUser = await fetchJSON(`/api/auth/users/${currentUser.id}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          ...getAuthHeaders(),
        },
        body: JSON.stringify(payload),
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
  }, [currentUser, isSavingProfile, profileFormData.email, profileFormData.password, profileFormData.username, t]);

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

  // Render navigation item
  const renderNavItem = (item, mobile = false) => {
    const isActive = activeNav === item.id;
    const baseClasses = "no-underline rounded transition-colors cursor-pointer border-0 font-medium " + (item.baseClasses || "");
    const desktopClasses = "px-3 py-2";
    const mobileClasses = "block w-full px-4 py-3";
    const activeClass = isActive ? 'bg-[hsl(var(--primary))] text-[hsl(var(--primary-foreground))]' : 'text-[hsl(var(--card-foreground))] hover:bg-[hsl(var(--primary)/0.8)] hover:text-[hsl(var(--primary-foreground))]';

    const isDisabled = item.disabled === true;
    return (
        <li className={mobile ? "" : item.classNameLi ? item.classNameLi : "mx-1"}>
          <a
              href={isDisabled ? undefined : item.href}
              id={item.id}
              title={item.title}
              aria-disabled={isDisabled || undefined}
              className={`${baseClasses} ${mobile ? mobileClasses : desktopClasses} ${activeClass}${isDisabled ? ' opacity-60 cursor-default pointer-events-none' : ''}`}
              onClick={(e) => {
                if (isDisabled) return;

                // Force navigation and prevent default behavior
                if (item.href) {
                  forceNavigation(item.href, e);
                }

                // Call onClick function if provided
                if (item.onClick) {
                  item.onClick(e);
                }

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
    return renderNavItem({
      id: 'nav-editProfile',
      title: t('auth.editProfile'),
      disabled: !canEditCurrentUser,
      onClick: openProfileModal,
      label: displayUsername,
      classNameLi: 'ml-3',
      baseClasses: mobile ? 'text-center' : '',
    }, mobile);
  };

  const renderLoginLogout = (login = false, mobile = false) => {
    return renderNavItem({
      id: login ? 'nav-login' : 'nav-logout',
      href: login ? "/login.html" : "/logout",
      label: login ? t('auth.login') : t('auth.logout'),
      classNameLi: 'ml-2',
      baseClasses: (mobile ? 'text-right ' : '') + (login ? 'login-link' : 'logout-link'),
    }, mobile);
  };

  return (
      <>
      <header className="app-header py-2 shadow-md mb-4 w-full" style={{ position: 'relative', zIndex: 20, backgroundColor: 'hsl(var(--card))', color: 'hsl(var(--card-foreground))' }}>
        <div className="container mx-auto px-4 flex justify-between items-center">
          <a
            href={getLiveViewHref()}
            id="nav-logo"
            className="logo flex items-center no-underline text-inherit rounded hover:opacity-90 focus:outline-none focus:ring-2 focus:ring-[hsl(var(--primary))] focus:ring-offset-2"
            style={{ color: 'inherit' }}
            aria-label={t('nav.live')}
            onClick={(e) => {
              forceNavigation(getLiveViewHref(), e);
              if (mobileMenuOpen) {
                toggleMobileMenu();
              }
            }}
          >
            <h1 className="text-xl font-bold m-0">LightNVR</h1>
            <span className="version text-xs ml-2" style={{color: 'hsl(var(--muted-foreground))'}}>v{version}</span>
          </a>

          {/* Desktop Navigation */}
          <nav className="hidden lg:block" style={{ position: 'relative', zIndex: 20 }}>
            <ul className="flex list-none m-0 p-0">
              {navItems.map((navItem) => renderNavItem(navItem))}
            </ul>
          </nav>

          {/* User Menu (Desktop) */}
          <nav className="hidden xl:block" style={{ position: 'relative', zIndex: 20 }}>
            <ul className="flex list-none m-0 p-0 user-menu items-center">
              <LanguageSelector/>
              {demoMode && !localStorage.getItem('auth') && (
                <span className="mr-2 px-2 py-0.5 text-xs rounded" style={{backgroundColor: 'hsl(var(--accent))', color: 'hsl(var(--accent-foreground))'}}>{t('auth.demoMode')}</span>
              )}
              {renderUsername()}
              {authEnabled && (
                demoMode && !localStorage.getItem('auth') ? renderLoginLogout(true) : renderLoginLogout(false)
              )}
            </ul>
          </nav>

          {/* Mobile Menu Button */}
          <button
              className="xl:hidden p-2 focus:outline-none"
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
            <div className="xl:hidden mt-2 border-t pt-2 container mx-auto px-4" style={{borderColor: 'hsl(var(--border))'}}>
              <ul className="list-none m-0 p-0 flex flex-col w-full">
                {/* Main nav links — only shown on screens too narrow for Desktop Navigation */}
                <li className="lg:hidden border-b pb-2 w-full" style={{borderColor: 'hsl(var(--border))'}}>
                  <ul className="list-none m-0 p-0 flex flex-col w-full">
                    {navItems.map((navItem) => renderNavItem(navItem, true))}
                  </ul>
                </li>
                {/* User / language section — always shown in mobile menu */}
                <li className="w-full">
                  <ul className="list-none m-0 p-0 flex flex-row flex-wrap items-center justify-between px-4 py-2 w-full">
                    <li><LanguageSelector mobile={true}/></li>
                    {authEnabled && (
                      <>
                        {demoMode && !localStorage.getItem('auth') && (
                          <li>
                            <span className="mr-2 px-2 py-0.5 text-xs rounded" style={{backgroundColor: 'hsl(var(--accent))', color: 'hsl(var(--accent-foreground))'}}>{t('auth.demoShort')}</span>
                          </li>
                        )}
                        {renderUsername(true)}
                        {demoMode && !localStorage.getItem('auth') ? renderLoginLogout(true, true) : renderLoginLogout(false, true)}
                      </>
                    )}
                  </ul>
                </li>
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
          showPasswordField={true}
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