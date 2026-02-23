/**
 * ClearLogsModal Component
 * A styled confirmation modal for clearing logs
 */

import { useRef } from 'preact/hooks';
import { createPortal } from 'preact/compat';

/**
 * ClearLogsModal component
 * @param {Object} props Component props
 * @param {boolean} props.isOpen Whether the modal is open
 * @param {Function} props.onClose Function to close the modal
 * @param {Function} props.onConfirm Function to call when clear is confirmed
 * @returns {JSX.Element} ClearLogsModal component
 */
export function ClearLogsModal({ isOpen, onClose, onConfirm }) {
  const modalRef = useRef(null);

  if (!isOpen) return null;

  const handleBackgroundClick = (e) => {
    if (e.target === modalRef.current) {
      onClose();
    }
  };

  const handleConfirm = () => {
    onConfirm();
    onClose();
  };

  return createPortal(
    <div
      ref={modalRef}
      className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
      onClick={handleBackgroundClick}
    >
      <div className="bg-card text-card-foreground rounded-lg shadow-xl p-6 max-w-md w-full mx-4 transform transition-all duration-300 ease-out">
        <div className="flex items-center mb-4">
          <div className="flex-shrink-0 w-12 h-12 rounded-full bg-red-100 dark:bg-red-900/30 flex items-center justify-center mr-4">
            <svg className="w-6 h-6 text-red-600 dark:text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
          </div>
          <div>
            <h3 className="text-lg font-semibold">Clear Logs</h3>
            <p className="text-sm text-muted-foreground">This action cannot be undone</p>
          </div>
        </div>

        <p className="text-muted-foreground mb-6">
          Are you sure you want to clear all logs? The log file will be permanently erased.
        </p>

        <div className="flex justify-end space-x-3">
          <button
            className="px-4 py-2 border border-border rounded-md text-sm font-medium hover:bg-muted transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary dark:focus:ring-offset-gray-800"
            onClick={onClose}
          >
            Cancel
          </button>
          <button
            className="btn-danger px-4 py-2 rounded-md text-sm font-medium focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
            onClick={handleConfirm}
          >
            Clear Logs
          </button>
        </div>
      </div>
    </div>,
    document.body
  );
}

