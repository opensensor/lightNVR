const CATEGORY_STYLES = {
  'Application': 'bg-blue-500/10 text-blue-700 dark:text-blue-300',
  'OS': 'bg-violet-500/10 text-violet-700 dark:text-violet-300',
  'Service': 'bg-amber-500/10 text-amber-700 dark:text-amber-300',
  'Library': 'bg-emerald-500/10 text-emerald-700 dark:text-emerald-300'
};

function CategoryBadge({ category }) {
  const className = CATEGORY_STYLES[category] || 'bg-muted text-muted-foreground';
  return (
    <span className={`inline-flex rounded-full px-2.5 py-1 text-xs font-semibold ${className}`}>
      {category || 'Other'}
    </span>
  );
}

export function VersionsTable({ versions }) {
  const items = Array.isArray(versions?.items) ? versions.items : [];

  return (
    <div className="bg-card text-card-foreground rounded-lg shadow p-4" data-testid="versions-panel">
      <div className="flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between mb-4">
        <div>
          <h3 className="text-lg font-semibold">Versions</h3>
          <p className="text-sm text-muted-foreground">
            Runtime summary of the software LightNVR depends on.
          </p>
        </div>
        <span className="text-sm text-muted-foreground">{items.length} entries</span>
      </div>

      {items.length === 0 ? (
        <div className="rounded-md border border-border p-4 text-sm text-muted-foreground">
          Version information is not available yet.
        </div>
      ) : (
        <div className="overflow-x-auto" data-testid="versions-table">
          <table className="min-w-full text-sm">
            <thead className="text-left text-muted-foreground border-b border-border">
              <tr>
                <th className="py-2 pr-4 font-medium">Software</th>
                <th className="py-2 pr-4 font-medium">Category</th>
                <th className="py-2 pr-4 font-medium">Version</th>
                <th className="py-2 font-medium">Details</th>
              </tr>
            </thead>
            <tbody>
              {items.map((item, index) => (
                <tr
                  key={`${item.category}-${item.name}-${item.version || 'unknown'}`}
                  className="border-b border-border/60 align-top last:border-b-0"
                  data-testid="version-row"
                >
                  <td className="py-3 pr-4 font-medium whitespace-nowrap">{item.name}</td>
                  <td className="py-3 pr-4 whitespace-nowrap">
                    <CategoryBadge category={item.category} />
                  </td>
                  <td className="py-3 pr-4 font-mono text-xs sm:text-sm whitespace-nowrap">
                    {item.version || 'Unknown'}
                  </td>
                  <td className="py-3 text-muted-foreground">{item.details || '—'}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}