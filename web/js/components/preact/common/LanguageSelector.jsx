import { getLocalesUrl, useI18n } from '../../../i18n.js';

export default function LanguageSelector({ mobile = false }) {
  const { t, locale, localePreference, setLocalePreference, availableLocales } = useI18n();

  const renderLocaleContent = (item, menu = false) => (
    <>
      <img
        width="40"
        height="22"
        className="p-0 ml-3"
        src={getLocalesUrl(item.code + '.png')}
        alt={`${item.code.toUpperCase()} (${item.nativeName})`}
      />
      <span
        style={menu ? { whiteSpace: 'nowrap' } : {}}
        className={menu ? 'ml-1' : 'ml-3 mr-3'}
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
      <button type="button" aria-label={t('language.label')} className={`flex cursor-pointer border-0 font-medium items-center ${mobile ? 'w-full px-4 py-3' : 'py-1.5'} focus-visible:outline focus-visible:outline-2 focus-visible:outline-primary`}>
        {currentLocale && renderLocaleContent(currentLocale)}
      </button>
      <ul
        tabIndex={-1}
        className="rounded transition-colors dropdown-content menu bg-[hsl(var(--card))]"
        style={{ width: 'max-content', border: '1px solid hsl(var(--primary)/0.8)' }}
      >
        <li key="auto">
          <button type="button"
            style={{ flexFlow: 'row nowrap' }}
            className={`flex cursor-pointer border-0 font-medium items-center ${baseClasses} py-1 ${activeClass} focus-visible:outline focus-visible:outline-2 focus-visible:outline-primary`}
            onClick={() => {
              setLocalePreference(null);
              document.activeElement.blur();
            }}
          >
            <span style={{ whiteSpace: 'nowrap' }} className="ml-3 mr-3">{t('language.browserDefault')}</span>
          </button>
        </li>
        {availableLocales.map((item) => (
          <li key={item.code}>
            <button type="button" aria-label={`${item.code.toUpperCase()} (${item.nativeName})`}
              style={{ flexFlow: 'row nowrap' }}
              className={`flex cursor-pointer border-0 font-medium items-center ${baseClasses} py-1 ${activeClass} focus-visible:outline focus-visible:outline-2 focus-visible:outline-primary`}
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

