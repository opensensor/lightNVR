import { getLocalesUrl, useI18n } from '../../../i18n.js';

export default function LanguageSelector({ mobile = false }) {
  const { t, locale, setLocalePreference, availableLocales } = useI18n();

  const renderLocaleContent = (item, menu = false) => (
    <>
      <img
        width="40"
        height="22"
        className="p-0 ms-3"
        src={getLocalesUrl(item.code + '.png')}
        alt={`${item.code.toUpperCase()} (${item.nativeName})`}
      />
      <span
        style={menu ? { whiteSpace: 'nowrap' } : {}}
        className={menu ? 'ms-1' : 'ms-3 me-3'}
      >
        {item.code.toUpperCase() + (menu ? ` (${item.nativeName})` : '')}
      </span>
    </>
  );

  const baseClasses = "no-underline rounded transition-colors";
  const activeClass = 'text-[hsl(var(--card-foreground))] hover:bg-[hsl(var(--primary)/0.8)] hover:text-[hsl(var(--primary-foreground))]';

  const currentLocale = availableLocales.find((item) => item.code === locale) || availableLocales[0];

  const inner = (
    <div className={`dropdown ${baseClasses} ${activeClass}${mobile ? ' w-full' : ''}`}>
      {/* daisyUI focus-based dropdown: the trigger must be a focusable
          non-<button> element, because Safari does not focus <button> on
          click and the menu would never open there. */}
      <div
        tabIndex={0}
        role="button"
        aria-haspopup="true"
        className={`flex cursor-pointer border-0 font-medium items-center ${mobile ? 'w-full px-4 py-3' : 'py-1.5'}`}
      >
        {currentLocale && renderLocaleContent(currentLocale)}
      </div>
      <ul
        tabIndex={-1}
        className="rounded transition-colors dropdown-content menu bg-[hsl(var(--card))]"
        style={{ width: 'max-content', border: '1px solid hsl(var(--primary)/0.8)' }}
      >
        <li key="auto">
          <button
            type="button"
            className={`flex w-full cursor-pointer border-0 font-medium items-center ${baseClasses} py-1 ${activeClass}`}
            onClick={() => {
              setLocalePreference(null);
              document.activeElement.blur();
            }}
          >
            <span style={{ whiteSpace: 'nowrap' }} className="ms-3 me-3">{t('language.browserDefault')}</span>
          </button>
        </li>
        {availableLocales.map((item) => (
          <li key={item.code}>
            <button
              type="button"
              className={`flex w-full cursor-pointer border-0 font-medium items-center ${baseClasses} py-1 ${activeClass}`}
              onClick={() => {
                setLocalePreference(item.code);
                document.activeElement.blur();
              }}
            >
              {renderLocaleContent(item, true)}
            </button>
          </li>
        ))}
      </ul>
    </div>
  );

  if (mobile) {
    return inner;
  }

  return <li className="mx-1">{inner}</li>;
}

