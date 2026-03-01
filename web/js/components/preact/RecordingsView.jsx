/**
 * LightNVR Web Interface RecordingsView Component
 * Preact component for the recordings page
 */

import { useState, useEffect, useRef, useContext } from 'preact/hooks';
import { useQueryClient } from '../../query-client.js';
import { showStatusMessage } from './ToastContainer.jsx';
import { showVideoModal, DeleteConfirmationModal, ModalContext } from './UI.jsx';
import { BatchDownloadModal } from './BatchDownloadModal.jsx';
import { ContentLoader } from './LoadingIndicator.jsx';
import { clearThumbnailQueue } from '../../request-queue.js';

// Import components
import { FiltersSidebar } from './recordings/FiltersSidebar.jsx';
import { ActiveFilters } from './recordings/ActiveFilters.jsx';
import { RecordingsTable } from './recordings/RecordingsTable.jsx';
import { RecordingsGrid } from './recordings/RecordingsGrid.jsx';
import { PaginationControls } from './recordings/PaginationControls.jsx';

// Import utilities
import { formatUtils } from './recordings/formatUtils.js';
import { recordingsAPI } from './recordings/recordingsAPI.jsx';
import { urlUtils } from './recordings/urlUtils.js';

import { validateSession } from '../../utils/auth-utils.js';

/**
 * RecordingsView component
 * @returns {JSX.Element} RecordingsView component
 */
export function RecordingsView() {
  const queryClient = useQueryClient();
  const [userRole, setUserRole] = useState(null);
  const [recordings, setRecordings] = useState([]);
  const [streams, setStreams] = useState([]);

  // Initialize sort state from URL params (lazy — no double render)
  const [sortField, setSortField] = useState(() => {
    const p = new URLSearchParams(window.location.search);
    return p.get('sort') || 'start_time';
  });
  const [sortDirection, setSortDirection] = useState(() => {
    const p = new URLSearchParams(window.location.search);
    return p.get('order') || 'desc';
  });

  // Initialize filter state from URL params (lazy — no double render)
  const [filters, setFilters] = useState(() => {
    const p = new URLSearchParams(window.location.search);
    const hasUrlParams = p.has('dateRange') || p.has('page') || p.has('sort') ||
      p.has('detection') || p.has('stream') || p.has('detection_label');

    if (!hasUrlParams) {
      // No URL params — use defaults with current date range
      const now = new Date();
      const sevenDaysAgo = new Date(now);
      sevenDaysAgo.setDate(now.getDate() - 7);
      return {
        dateRange: 'last7days',
        startDate: sevenDaysAgo.toISOString().split('T')[0],
        startTime: '00:00',
        endDate: now.toISOString().split('T')[0],
        endTime: '23:59',
        streamId: 'all',
        recordingType: 'all',
        detectionLabel: ''
      };
    }

    return {
      dateRange: p.get('dateRange') || 'last7days',
      startDate: p.get('startDate') || '',
      startTime: p.get('startTime') || '00:00',
      endDate: p.get('endDate') || '',
      endTime: p.get('endTime') || '23:59',
      streamId: p.get('stream') || 'all',
      recordingType: p.get('detection') === '1' ? 'detection' : 'all',
      detectionLabel: p.get('detection_label') || ''
    };
  });

  // Initialize pagination state from URL params (lazy — no double render)
  const [pagination, setPagination] = useState(() => {
    const p = new URLSearchParams(window.location.search);
    return {
      currentPage: parseInt(p.get('page') || '1', 10),
      pageSize: parseInt(p.get('limit') || '20', 10),
      totalItems: 0,
      totalPages: 1,
      startItem: 0,
      endItem: 0
    };
  });
  const [hasActiveFilters, setHasActiveFilters] = useState(false);
  const [activeFiltersDisplay, setActiveFiltersDisplay] = useState([]);
  const [selectedRecordings, setSelectedRecordings] = useState({});
  const [selectAll, setSelectAll] = useState(false);
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false);
  const [deleteMode, setDeleteMode] = useState('selected'); // 'single', 'selected' or 'all'
  const [pendingDeleteRecording, setPendingDeleteRecording] = useState(null); // recording awaiting single-delete confirmation
  const [isDownloadModalOpen, setIsDownloadModalOpen] = useState(false);
  const recordingsTableBodyRef = useRef(null);

  // View mode: 'table' or 'grid' — initialized from URL, then localStorage, then default
  const [viewMode, setViewMode] = useState(() => {
    const p = new URLSearchParams(window.location.search);
    const urlView = p.get('view');
    if (urlView === 'grid' || urlView === 'table') return urlView;
    return localStorage.getItem('recordings_view_mode') || 'table';
  });
  const [thumbnailsEnabled, setThumbnailsEnabled] = useState(true);

  // Column visibility for table view
  const [hiddenColumns, setHiddenColumns] = useState(() => {
    try {
      const saved = localStorage.getItem('recordings_hidden_columns');
      return saved ? JSON.parse(saved) : {};
    } catch { return {}; }
  });

  const toggleColumn = (col) => {
    setHiddenColumns(prev => {
      const next = { ...prev, [col]: !prev[col] };
      localStorage.setItem('recordings_hidden_columns', JSON.stringify(next));
      return next;
    });
  };

  // Persist view mode preference
  const handleViewModeChange = (mode) => {
    setViewMode(mode);
    localStorage.setItem('recordings_view_mode', mode);
  };

  // Fetch generate_thumbnails setting
  useEffect(() => {
    fetch('/api/settings')
      .then(res => res.json())
      .then(data => {
        if (data && typeof data.generate_thumbnails !== 'undefined') {
          setThumbnailsEnabled(data.generate_thumbnails);
          // If thumbnails disabled and user had grid mode saved, fall back to table
          if (!data.generate_thumbnails && viewMode === 'grid') {
            handleViewModeChange('table');
          }
        }
      })
      .catch(() => {}); // Silently ignore - default to enabled
  }, []);

  // Get modal context for video playback
  const modalContext = useContext(ModalContext);

  // Fetch user role on mount
  useEffect(() => {
    async function fetchUserRole() {
      const session = await validateSession();
      if (session.valid) {
        setUserRole(session.role);
        console.log('User role:', session.role);
      } else {
        // Session invalid - set to empty string to indicate fetch completed
        setUserRole('');
        console.log('Session validation failed, no role');
      }
    }
    fetchUserRole();
  }, []);

  // Role is still loading if null
  const roleLoading = userRole === null;
  // Check if user can delete recordings (admin or user role, not viewer)
  // While loading, keep delete disabled until authentication is verified
  const canDelete = !roleLoading && (userRole === 'admin' || userRole === 'user');

  // Store modal context in window for global access
  useEffect(() => {
    if (modalContext) {
      console.log('Modal context available in RecordingsView');
      window.__modalContext = modalContext;

      // Log the available methods for debugging
      console.log('Available modal methods:',
        Object.keys(modalContext).map(key => key)
      );
    } else {
      console.warn('Modal context not available in RecordingsView');
    }
  }, [modalContext]);

  // Fetch streams using preact-query
  const {
    data: streamsData,
    error: streamsError
  } = recordingsAPI.hooks.useStreams({
    // Streams are relatively static; avoid unnecessary refetches
    staleTime: 5 * 60 * 1000,  // 5 minutes
    cacheTime: 10 * 60 * 1000  // 10 minutes
  });

  // Update streams state when data is loaded
  useEffect(() => {
    if (streamsData && Array.isArray(streamsData)) {
      setStreams(streamsData);
    }
  }, [streamsData]);

  // Handle streams error
  useEffect(() => {
    if (streamsError) {
      console.error('Error loading streams for filter:', streamsError);
      showStatusMessage('Error loading streams: ' + streamsError.message);
    }
  }, [streamsError]);

  // Clear thumbnail queue when component unmounts (user navigates away)
  useEffect(() => {
    return () => {
      clearThumbnailQueue();
    };
  }, []);

  // Update active filters when filters change
  useEffect(() => {
    updateActiveFilters();
  }, [filters]);

  // Reactively sync all view state to URL via replaceState (no browser history entries).
  // This mirrors the approach used in LiveView and ensures refresh always preserves state.
  useEffect(() => {
    const url = new URL(window.location.href);

    // Filters
    url.searchParams.set('dateRange', filters.dateRange);
    if (filters.dateRange === 'custom') {
      url.searchParams.set('startDate', filters.startDate);
      url.searchParams.set('startTime', filters.startTime);
      url.searchParams.set('endDate', filters.endDate);
      url.searchParams.set('endTime', filters.endTime);
    } else {
      url.searchParams.delete('startDate');
      url.searchParams.delete('startTime');
      url.searchParams.delete('endDate');
      url.searchParams.delete('endTime');
    }

    if (filters.streamId !== 'all') url.searchParams.set('stream', filters.streamId);
    else url.searchParams.delete('stream');

    if (filters.recordingType === 'detection') url.searchParams.set('detection', '1');
    else url.searchParams.delete('detection');

    if (filters.detectionLabel && filters.detectionLabel.trim()) {
      url.searchParams.set('detection_label', filters.detectionLabel.trim());
    } else {
      url.searchParams.delete('detection_label');
    }

    // Pagination
    url.searchParams.set('page', pagination.currentPage.toString());
    url.searchParams.set('limit', pagination.pageSize.toString());

    // Sort
    url.searchParams.set('sort', sortField);
    url.searchParams.set('order', sortDirection);

    // View mode — omit param when 'table' (the default) to keep URLs clean
    if (viewMode !== 'table') url.searchParams.set('view', viewMode);
    else url.searchParams.delete('view');

    window.history.replaceState({}, '', url);
  }, [filters, pagination.currentPage, pagination.pageSize, sortField, sortDirection, viewMode]);

  // Set default date range (used when switching to 'custom' with no existing dates)
  const setDefaultDateRange = () => {
    const now = new Date();
    const sevenDaysAgo = new Date(now);
    sevenDaysAgo.setDate(now.getDate() - 7);

    setFilters(prev => ({
      ...prev,
      endDate: now.toISOString().split('T')[0],
      startDate: sevenDaysAgo.toISOString().split('T')[0]
    }));
  };

  // Fetch recordings using preact-query
  const {
    data: recordingsData,
    isLoading: isLoadingRecordings,
    error: recordingsError
  } = recordingsAPI.hooks.useRecordings(filters, pagination, sortField, sortDirection);

  // Update recordings state when data is loaded
  useEffect(() => {
    if (recordingsData) {
      // Store recordings in the component state
      const recordingsArray = recordingsData.recordings || [];

      // When filtering for detection events, all returned recordings should have detections
      if (filters.recordingType === 'detection') {
        recordingsArray.forEach(recording => {
          recording.has_detections = true;
        });
      }

      // Set the recordings state
      setRecordings(recordingsArray);
      setHasData(recordingsArray.length > 0);

      // Update pagination
      if (recordingsData.pagination) {
        updatePaginationFromResponse(recordingsData, pagination.currentPage);
      }
    }
  }, [recordingsData, filters.recordingType, pagination.currentPage]);

  // Handle recordings error
  useEffect(() => {
    if (recordingsError) {
      console.error('Error loading recordings:', recordingsError);
      showStatusMessage('Error loading recordings: ' + recordingsError.message);
      setHasData(false);
    }
  }, [recordingsError]);


  // State for data status
  const [hasData, setHasData] = useState(false);

  // Load recordings — updates page in state; URL sync is handled by the reactive useEffect
  const loadRecordings = (page = pagination.currentPage) => {
    setPagination(prev => ({ ...prev, currentPage: page }));
  };

  // Update pagination from API response
  const updatePaginationFromResponse = (data, currentPage) => {
    // Use the provided page parameter instead of the state
    currentPage = currentPage || pagination.currentPage;

    if (data.pagination) {
      const pageSize = data.pagination.limit || 20;
      const totalItems = data.pagination.total || 0;
      const totalPages = data.pagination.pages || 1;

      // Calculate start and end items based on current page
      let startItem = 0;
      let endItem = 0;

      if (data.recordings.length > 0) {
        startItem = (currentPage - 1) * pageSize + 1;
        endItem = Math.min(startItem + data.recordings.length - 1, totalItems);
      }

      console.log('Pagination update:', {
        currentPage,
        pageSize,
        totalItems,
        totalPages,
        startItem,
        endItem,
        recordingsLength: data.recordings.length
      });

      setPagination(prev => ({
        ...prev,
        totalItems,
        totalPages,
        pageSize,
        startItem,
        endItem
      }));
    } else {
      // Fallback if pagination object is not provided
      const pageSize = pagination.pageSize;
      const totalItems = data.total || 0;
      const totalPages = Math.ceil(totalItems / pageSize) || 1;

      // Calculate start and end items based on current page
      let startItem = 0;
      let endItem = 0;

      if (data.recordings.length > 0) {
        startItem = (currentPage - 1) * pageSize + 1;
        endItem = Math.min(startItem + data.recordings.length - 1, totalItems);
      }

      console.log('Pagination update (fallback):', {
        currentPage,
        pageSize,
        totalItems,
        totalPages,
        startItem,
        endItem,
        recordingsLength: data.recordings.length
      });

      setPagination(prev => ({
        ...prev,
        totalItems,
        totalPages,
        startItem,
        endItem
      }));
    }
  };

  // Handle date range change
  const handleDateRangeChange = (e) => {
    const newDateRange = e.target.value;

    setFilters(prev => ({
      ...prev,
      dateRange: newDateRange
    }));

    if (newDateRange === 'custom') {
      // If custom is selected, make sure we have default dates
      if (!filters.startDate || !filters.endDate) {
        const now = new Date();
        const sevenDaysAgo = new Date(now);
        sevenDaysAgo.setDate(now.getDate() - 7);

        setFilters(prev => ({
          ...prev,
          endDate: now.toISOString().split('T')[0],
          startDate: sevenDaysAgo.toISOString().split('T')[0]
        }));
      }
    }
  };

  // Update active filters
  const updateActiveFilters = () => {
    const activeFilters = urlUtils.getActiveFiltersDisplay(filters);
    setHasActiveFilters(activeFilters.length > 0);
    setActiveFiltersDisplay(activeFilters);
  };

  // Apply filters — resets to page 1; URL sync is handled by the reactive useEffect
  const applyFilters = (resetToFirstPage = true) => {
    if (resetToFirstPage) {
      setPagination(prev => ({ ...prev, currentPage: 1 }));
    }
    // URL sync handled by the reactive useEffect
  };

  // Reset filters — resets all state to defaults; URL sync is handled by the reactive useEffect
  const resetFilters = () => {
    const now = new Date();
    const sevenDaysAgo = new Date(now);
    sevenDaysAgo.setDate(now.getDate() - 7);

    setFilters({
      dateRange: 'last7days',
      startDate: sevenDaysAgo.toISOString().split('T')[0],
      startTime: '00:00',
      endDate: now.toISOString().split('T')[0],
      endTime: '23:59',
      streamId: 'all',
      recordingType: 'all',
      detectionLabel: ''
    });
    setPagination(prev => ({ ...prev, currentPage: 1 }));
    setSortField('start_time');
    setSortDirection('desc');
    // URL sync handled by the reactive useEffect
  };

  // Remove filter
  const removeFilter = (key) => {
    switch (key) {
      case 'dateRange':
        setFilters(prev => ({
          ...prev,
          dateRange: 'last7days'
        }));
        break;
      case 'streamId':
        setFilters(prev => ({
          ...prev,
          streamId: 'all'
        }));
        break;
      case 'recordingType':
        setFilters(prev => ({
          ...prev,
          recordingType: 'all'
        }));
        break;
      case 'detectionLabel':
        setFilters(prev => ({
          ...prev,
          detectionLabel: ''
        }));
        break;
    }

    applyFilters();
  };

  // Sort by field — URL sync is handled by the reactive useEffect
  const sortBy = (field) => {
    if (sortField === field) {
      setSortDirection(sortDirection === 'asc' ? 'desc' : 'asc');
    } else {
      setSortDirection(field === 'start_time' ? 'desc' : 'asc');
      setSortField(field);
    }
    setPagination(prev => ({ ...prev, currentPage: 1 }));
    // URL sync handled by the reactive useEffect
  };

  // Go to page — URL sync is handled by the reactive useEffect
  const goToPage = (page) => {
    if (page < 1 || page > pagination.totalPages) return;
    clearThumbnailQueue();
    setPagination(prev => ({ ...prev, currentPage: page }));
    // URL sync handled by the reactive useEffect
  };

  // Toggle selection of a recording
  const toggleRecordingSelection = (recordingId) => {
    setSelectedRecordings(prev => ({
      ...prev,
      [recordingId]: !prev[recordingId]
    }));
  };

  // Toggle select all recordings
  const toggleSelectAll = () => {
    const newSelectAll = !selectAll;
    setSelectAll(newSelectAll);

    const newSelectedRecordings = {};
    if (newSelectAll) {
      // Select all recordings
      recordings.forEach(recording => {
        newSelectedRecordings[recording.id] = true;
      });
    }
    // Always update selectedRecordings, even when deselecting all
    setSelectedRecordings(newSelectedRecordings);
  };

  // Get count of selected recordings
  const getSelectedCount = () => {
    return Object.values(selectedRecordings).filter(Boolean).length;
  };

  // Clear all selections
  const clearSelections = () => {
    setSelectedRecordings({});
    setSelectAll(false);
  };

  // Open download modal
  const openDownloadModal = () => setIsDownloadModalOpen(true);

  // Open delete confirmation modal
  const openDeleteModal = (mode) => {
    setDeleteMode(mode);
    setIsDeleteModalOpen(true);
  };

  // Close delete confirmation modal
  const closeDeleteModal = () => {
    setIsDeleteModalOpen(false);
    setPendingDeleteRecording(null);
  };

  // Handle delete confirmation
  const handleDeleteConfirm = async () => {
    closeDeleteModal();

    if (deleteMode === 'single' && pendingDeleteRecording) {
      // Single recording delete via mutation; reload list on success
      deleteRecordingMutation(pendingDeleteRecording.id, {
        onSuccess: () => loadRecordings()
      });
      setPendingDeleteRecording(null);
      return;
    }

    // Save current URL parameters before deletion
    const currentUrlParams = new URLSearchParams(window.location.search);
    const currentSortField = currentUrlParams.get('sort') || sortField;
    const currentSortDirection = currentUrlParams.get('order') || sortDirection;
    const currentPage = parseInt(currentUrlParams.get('page'), 10) || pagination.currentPage;

    if (deleteMode === 'selected') {
      // Use the recordingsAPI to delete selected recordings
      const result = await recordingsAPI.deleteSelectedRecordings(selectedRecordings);

      // Reset selection
      setSelectedRecordings({});
      setSelectAll(false);

      // Only reload if some recordings were deleted successfully
      if (result.succeeded > 0) {
        // Reload recordings with preserved parameters
        reloadRecordingsWithPreservedParams(currentSortField, currentSortDirection, currentPage);
      }
    } else {
      // Use the recordingsAPI to delete all filtered recordings
      await recordingsAPI.deleteAllFilteredRecordings(filters);

      // Reset selection
      setSelectedRecordings({});
      setSelectAll(false);

      // Invalidate the recordings query cache so the list re-fetches immediately,
      // regardless of staleTime. This is necessary because loadRecordings() is a
      // no-op when the page hasn't changed, and the cache may still hold the old data.
      queryClient.invalidateQueries({ queryKey: ['recordings'] });
    }
  };

  // Helper function to reload recordings with preserved parameters
  const reloadRecordingsWithPreservedParams = (sortField, sortDirection, page) => {
    // Set the sort parameters directly
    setSortField(sortField);
    setSortDirection(sortDirection);

    // Update pagination with the preserved page
    setPagination(prev => ({
      ...prev,
      currentPage: page
    }));

    // Load recordings from API with the updated parameters
    setTimeout(() => {
      const updatedPagination = { ...pagination, currentPage: page };
      // URL sync handled by the reactive useEffect
      recordingsAPI.loadRecordings(filters, updatedPagination, sortField, sortDirection)
        .then(data => {
          setRecordings(data.recordings || []);
          updatePaginationFromResponse(data, page);
        })
        .catch(error => {
          console.error('Error loading recordings:', error);
          showStatusMessage('Error loading recordings: ' + error.message);
        });
    }, 0);
  };

  // Delete recording using preact-query mutation
  const { mutate: deleteRecordingMutation } = recordingsAPI.hooks.useDeleteRecording();

  // Delete a single recording — opens the confirmation modal instead of window.confirm()
  const deleteRecording = (recording) => {
    setPendingDeleteRecording(recording);
    setDeleteMode('single');
    setIsDeleteModalOpen(true);
  };

  // Toggle recording protection
  const toggleProtection = async (recording) => {
    const newProtectedState = !recording.protected;
    try {
      const response = await fetch(`/api/recordings/${recording.id}/protect`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ protected: newProtectedState }),
      });

      if (!response.ok) {
        throw new Error(`Failed to ${newProtectedState ? 'protect' : 'unprotect'} recording`);
      }

      // Update the local state
      setRecordings(prevRecordings =>
        prevRecordings.map(r =>
          r.id === recording.id ? { ...r, protected: newProtectedState } : r
        )
      );

      showStatusMessage(
        newProtectedState
          ? 'Recording protected from automatic deletion'
          : 'Recording protection removed',
        'success'
      );
    } catch (error) {
      console.error('Error toggling protection:', error);
      showStatusMessage(`Error: ${error.message}`, 'error');
    }
  };

  // Play recording
  const playRecording = (recording) => {
    console.log('RecordingsView.playRecording called with:', recording);

    // Use the modal context if available, otherwise fall back to the imported function
    if (modalContext && modalContext.showVideoModal) {
      console.log('Using modal context showVideoModal');
      const videoUrl = `/api/recordings/play/${recording.id}`;
      const title = `${recording.stream} - ${formatUtils.formatDateTime(recording.start_time)}`;
      const downloadUrl = `/api/recordings/download/${recording.id}`;

      // Call the context function directly
      modalContext.showVideoModal(videoUrl, title, downloadUrl);
    } else {
      console.log('Falling back to recordingsAPI.playRecording');
      // Fall back to the API function which will use the imported showVideoModal
      recordingsAPI.playRecording(recording, showVideoModal);
    }
  };

  // Download recording
  const downloadRecording = (recording) => {
    recordingsAPI.downloadRecording(recording);
  };

  return (
    <section id="recordings-page" class="page">
      <div class="page-header flex justify-between items-center mb-4 p-4 bg-card text-card-foreground rounded-lg shadow">
        <div class="flex items-center">
          <h2 class="text-xl font-bold">Recordings</h2>
          <div class="ml-4 flex">
            <button
              onClick={() => handleViewModeChange('table')}
              class="px-3 py-1 rounded-l-md text-sm"
              style={{
                backgroundColor: viewMode === 'table' ? 'hsl(var(--primary))' : 'hsl(var(--secondary))',
                color: viewMode === 'table' ? 'hsl(var(--primary-foreground))' : 'hsl(var(--secondary-foreground))'
              }}
            >
              Table
            </button>
            {thumbnailsEnabled && (
              <button
                onClick={() => handleViewModeChange('grid')}
                class="px-3 py-1 text-sm"
                style={{
                  backgroundColor: viewMode === 'grid' ? 'hsl(var(--primary))' : 'hsl(var(--secondary))',
                  color: viewMode === 'grid' ? 'hsl(var(--primary-foreground))' : 'hsl(var(--secondary-foreground))'
                }}
              >
                Grid
              </button>
            )}
            <a
              href="timeline.html"
              class="px-3 py-1 rounded-r-md text-sm"
              style={{
                backgroundColor: 'hsl(var(--secondary))',
                color: 'hsl(var(--secondary-foreground))'
              }}
              onMouseOver={(e) => e.currentTarget.style.backgroundColor = 'hsl(var(--secondary) / 0.8)'}
              onMouseOut={(e) => e.currentTarget.style.backgroundColor = 'hsl(var(--secondary))'}
            >
              Timeline
            </a>
          </div>
        </div>
      </div>

      <div class="recordings-layout flex flex-col md:flex-row gap-4 w-full">
        <FiltersSidebar
          filters={filters}
          setFilters={setFilters}
          pagination={pagination}
          setPagination={setPagination}
          streams={streams}
          applyFilters={applyFilters}
          resetFilters={resetFilters}
          handleDateRangeChange={handleDateRangeChange}
          setDefaultDateRange={setDefaultDateRange}
        />

        <div class="recordings-content flex-1">
          <ActiveFilters
            activeFiltersDisplay={activeFiltersDisplay}
            removeFilter={removeFilter}
            hasActiveFilters={hasActiveFilters}
          />

          <ContentLoader
            isLoading={isLoadingRecordings}
            hasData={hasData}
            loadingMessage="Loading recordings..."
            emptyMessage="No recordings found matching your criteria"
          >
            {viewMode === 'grid' && thumbnailsEnabled ? (
              <RecordingsGrid
                recordings={recordings}
                selectedRecordings={selectedRecordings}
                toggleRecordingSelection={toggleRecordingSelection}
                selectAll={selectAll}
                toggleSelectAll={toggleSelectAll}
                getSelectedCount={getSelectedCount}
                openDeleteModal={openDeleteModal}
                openDownloadModal={openDownloadModal}
                playRecording={playRecording}
                downloadRecording={downloadRecording}
                deleteRecording={deleteRecording}
                toggleProtection={toggleProtection}
                pagination={pagination}
                canDelete={canDelete}
                clearSelections={clearSelections}
                hiddenColumns={hiddenColumns}
                toggleColumn={toggleColumn}
              />
            ) : (
              <RecordingsTable
                recordings={recordings}
                sortField={sortField}
                sortDirection={sortDirection}
                sortBy={sortBy}
                selectedRecordings={selectedRecordings}
                toggleRecordingSelection={toggleRecordingSelection}
                selectAll={selectAll}
                toggleSelectAll={toggleSelectAll}
                getSelectedCount={getSelectedCount}
                openDeleteModal={openDeleteModal}
                openDownloadModal={openDownloadModal}
                playRecording={playRecording}
                downloadRecording={downloadRecording}
                deleteRecording={deleteRecording}
                toggleProtection={toggleProtection}
                recordingsTableBodyRef={recordingsTableBodyRef}
                pagination={pagination}
                canDelete={canDelete}
                hiddenColumns={hiddenColumns}
                toggleColumn={toggleColumn}
              />
            )}

            <PaginationControls
              pagination={pagination}
              goToPage={goToPage}
            />
          </ContentLoader>
        </div>
      </div>

      <DeleteConfirmationModal
        isOpen={isDeleteModalOpen}
        onClose={closeDeleteModal}
        onConfirm={handleDeleteConfirm}
        mode={deleteMode}
        count={getSelectedCount()}
        recordingName={pendingDeleteRecording?.stream}
      />

      <BatchDownloadModal
        isOpen={isDownloadModalOpen}
        onClose={() => setIsDownloadModalOpen(false)}
        recordings={recordings}
        selectedIds={selectedRecordings}
      />
    </section>
  );
}
