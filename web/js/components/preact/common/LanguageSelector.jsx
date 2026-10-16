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
      <div tabIndex={0} className={`flex cursor-pointer border-0 font-medium items-center ${mobile ? 'w-full px-4 py-3' : 'py-1.5'}`}>
        {currentLocale && renderLocaleContent(currentLocale)}
      </div>
      <ul
        tabIndex={-1}
        className="rounded transition-colors dropdown-content menu bg-[hsl(var(--card))]"
        style={{ width: 'max-content', border: '1px solid hsl(var(--primary)/0.8)' }}
      >
        <li
          key="auto"
          style={{ flexFlow: 'row nowrap' }}
          className={`flex cursor-pointer border-0 font-medium items-center ${baseClasses} py-1 ${activeClass}`}
          onClick={() => {
            setLocalePreference(null);
            document.activeElement.blur();
          }}
        >
          <span style={{ whiteSpace: 'nowrap' }} className="ml-3 mr-3">{t('language.browserDefault')}</span>
        </li>
        {availableLocales.map((item) => (
          <li
            key={item.code}
            style={{ flexFlow: 'row nowrap' }}
            className={`flex cursor-pointer border-0 font-medium items-center ${baseClasses} py-1 ${activeClass}`}
            onClick={() => {
              setLocalePreference(item.code);
              document.activeElement.blur();
            }}
          >
            {renderLocaleContent(item, true)}
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

