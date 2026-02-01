/**
 * RecordingsTable component for RecordingsView
 */

import { h } from 'preact';
import { formatUtils } from './formatUtils.js';

/**
 * RecordingsTable component
 * @param {Object} props Component props
 * @returns {JSX.Element} RecordingsTable component
 */
export function RecordingsTable({
  recordings,
  sortField,
  sortDirection,
  sortBy,
  selectedRecordings,
  toggleRecordingSelection,
  selectAll,
  toggleSelectAll,
  getSelectedCount,
  openDeleteModal,
  playRecording,
  downloadRecording,
  deleteRecording,
  toggleProtection,
  recordingsTableBodyRef,
  pagination,
  canDelete = true // Default to true for backward compatibility
}) {
  return (
    <div className="recordings-container bg-card text-card-foreground rounded-lg shadow overflow-hidden w-full">
      {/* Batch actions - only show if user can delete */}
      {canDelete && (
        <div className="batch-actions p-3 border-b border-border flex flex-wrap gap-2 items-center">
          <div className="selected-count text-sm text-muted-foreground mr-2">
            {getSelectedCount() > 0 ?
              `${getSelectedCount()} recording${getSelectedCount() !== 1 ? 's' : ''} selected` :
              'No recordings selected'}
          </div>
          <button
            className="btn-danger disabled:opacity-50 disabled:cursor-not-allowed"
            disabled={getSelectedCount() === 0}
            onClick={() => openDeleteModal('selected')}>
            Delete Selected
          </button>
          <button
            className="btn-danger"
            onClick={() => openDeleteModal('all')}>
            Delete All Filtered
          </button>
        </div>
      )}
      <div className="overflow-x-auto">
        <table id="recordings-table" className="min-w-full divide-y divide-border">
          <thead className="bg-muted">
            <tr>
              {/* Checkbox column - only show if user can delete */}
              {canDelete && (
                <th className="w-10 px-4 py-3">
                  <input
                    type="checkbox"
                    checked={selectAll}
                    onChange={toggleSelectAll}
                    className="w-4 h-4 rounded focus:ring-2"
                    style={{accentColor: 'hsl(var(--primary))'}}
                  />
                </th>
              )}
              <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider cursor-pointer"
                  onClick={() => sortBy('stream_name')}>
                <div className="flex items-center">
                  Stream
                  {sortField === 'stream_name' && (
                    <span className="sort-icon ml-1">{sortDirection === 'asc' ? '▲' : '▼'}</span>
                  )}
                </div>
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider cursor-pointer"
                  onClick={() => sortBy('start_time')}>
                <div className="flex items-center">
                  Start Time
                  {sortField === 'start_time' && (
                    <span className="sort-icon ml-1">{sortDirection === 'asc' ? '▲' : '▼'}</span>
                  )}
                </div>
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">
                Duration
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider cursor-pointer"
                  onClick={() => sortBy('size_bytes')}>
                <div className="flex items-center">
                  Size
                  {sortField === 'size_bytes' && (
                    <span className="sort-icon ml-1">{sortDirection === 'asc' ? '▲' : '▼'}</span>
                  )}
                </div>
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">
                Detections
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">
                Protected
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">
                Actions
              </th>
            </tr>
          </thead>
          <tbody ref={recordingsTableBodyRef} className="bg-card divide-y divide-border">
            {recordings.length === 0 ? (
              <tr>
                <td colSpan={canDelete ? 8 : 7} className="px-6 py-4 text-center text-muted-foreground">
                  {pagination.totalItems === 0 ? 'No recordings found' : 'Loading recordings...'}
                </td>
              </tr>
            ) : recordings.map(recording => (
              <tr key={recording.id} className="hover:bg-muted/50">
                {/* Row checkbox - only show if user can delete */}
                {canDelete && (
                  <td className="px-4 py-4 whitespace-nowrap">
                    <input
                      type="checkbox"
                      checked={!!selectedRecordings[recording.id]}
                      onChange={() => toggleRecordingSelection(recording.id)}
                      className="w-4 h-4 rounded focus:ring-2" style={{accentColor: 'hsl(var(--primary))'}}
                    />
                  </td>
                )}
                <td className="px-6 py-4 whitespace-nowrap">{recording.stream || ''}</td>
                <td className="px-6 py-4 whitespace-nowrap">{formatUtils.formatDateTime(recording.start_time)}</td>
                <td className="px-6 py-4 whitespace-nowrap">{formatUtils.formatDuration(recording.duration)}</td>
                <td className="px-6 py-4 whitespace-nowrap">{recording.size || ''}</td>
                <td className="px-6 py-4">
                  {recording.detection_labels && recording.detection_labels.length > 0 ? (
                    <div className="flex flex-wrap gap-1">
                      {recording.detection_labels.map((det, idx) => (
                        <span key={idx} className="badge-success text-xs" title={`${det.count} detection${det.count !== 1 ? 's' : ''}`}>
                          {det.label}
                          {det.count > 1 && <span className="ml-1 opacity-75">({det.count})</span>}
                        </span>
                      ))}
                    </div>
                  ) : recording.has_detections || recording.has_detection ? (
                    <span className="badge-success">
                      <svg className="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                        <path d="M10 12a2 2 0 100-4 2 2 0 000 4z"></path>
                        <path fillRule="evenodd" d="M.458 10C1.732 5.943 5.522 3 10 3s8.268 2.943 9.542 7c-1.274 4.057-5.064 7-9.542 7S1.732 14.057.458 10zM14 10a4 4 0 11-8 0 4 4 0 018 0z" clipRule="evenodd"></path>
                      </svg>
                      Yes
                    </span>
                  ) : ''}
                </td>
                <td className="px-6 py-4 whitespace-nowrap">
                  {recording.protected ? (
                    <button
                      className="badge-warning cursor-pointer hover:opacity-80"
                      onClick={() => toggleProtection && toggleProtection(recording)}
                      title="Click to unprotect"
                    >
                      <svg className="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                        <path fillRule="evenodd" d="M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z" clipRule="evenodd"></path>
                      </svg>
                      Protected
                    </button>
                  ) : (
                    <button
                      className="text-muted-foreground hover:text-foreground cursor-pointer text-xs"
                      onClick={() => toggleProtection && toggleProtection(recording)}
                      title="Click to protect from deletion"
                    >
                      <svg className="w-3 h-3 inline mr-1" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                        <path d="M10 2a5 5 0 00-5 5v2a2 2 0 00-2 2v5a2 2 0 002 2h10a2 2 0 002-2v-5a2 2 0 00-2-2H7V7a3 3 0 015.905-.75 1 1 0 001.937-.5A5.002 5.002 0 0010 2z"></path>
                      </svg>
                      Protect
                    </button>
                  )}
                </td>
                <td className="px-6 py-4 whitespace-nowrap">
                  <div className="flex space-x-2">
                    <button className="p-1 rounded-full focus:outline-none"
                            style={{color: 'hsl(var(--primary))'}}
                            onMouseOver={(e) => e.currentTarget.style.backgroundColor = 'hsl(var(--primary) / 0.1)'}
                            onMouseOut={(e) => e.currentTarget.style.backgroundColor = 'transparent'}
                            onClick={() => playRecording(recording)}
                            title="Play">
                      <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                        <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clipRule="evenodd"></path>
                      </svg>
                    </button>
                    <button className="p-1 rounded-full focus:outline-none"
                            style={{color: 'hsl(var(--success))'}}
                            onMouseOver={(e) => e.currentTarget.style.backgroundColor = 'hsl(var(--success) / 0.1)'}
                            onMouseOut={(e) => e.currentTarget.style.backgroundColor = 'transparent'}
                            onClick={() => downloadRecording(recording)}
                            title="Download">
                      <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                        <path fillRule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clipRule="evenodd"></path>
                      </svg>
                    </button>
                    {/* Delete button - only show if user can delete */}
                    {canDelete && (
                      <button className="p-1 rounded-full focus:outline-none"
                              style={{color: 'hsl(var(--danger))'}}
                              onMouseOver={(e) => e.currentTarget.style.backgroundColor = 'hsl(var(--danger) / 0.1)'}
                              onMouseOut={(e) => e.currentTarget.style.backgroundColor = 'transparent'}
                              onClick={() => deleteRecording(recording)}
                              title="Delete">
                        <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                          <path fillRule="evenodd" d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z" clipRule="evenodd"></path>
                        </svg>
                      </button>
                    )}
                  </div>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
