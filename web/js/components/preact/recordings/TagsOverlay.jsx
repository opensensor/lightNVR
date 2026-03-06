/**
 * TagsOverlay component for managing tags on recordings.
 * Shows current tags, allows adding/removing, with autocomplete from existing tags.
 */

import { useState, useEffect, useRef, useCallback } from 'preact/hooks';
import { recordingsAPI } from './recordingsAPI.jsx';
import { showStatusMessage } from '../ToastContainer.jsx';

/**
 * Tag icon SVG (label/tag shape)
 */
export function TagIcon({ className = 'w-4 h-4' }) {
  return (
    <svg className={className} fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2}
        d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z" />
    </svg>
  );
}

/**
 * Small tag chip for displaying a tag
 */
function TagChip({ tag, onRemove, readOnly = false }) {
  return (
    <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs bg-primary/10 text-primary border border-primary/20">
      {tag}
      {!readOnly && onRemove && (
        <button
          type="button"
          class="hover:text-destructive focus:outline-none ml-0.5"
          onClick={(e) => { e.stopPropagation(); onRemove(tag); }}
          title={`Remove tag "${tag}"`}
        >
          ×
        </button>
      )}
    </span>
  );
}

/**
 * Single-recording tag overlay
 */
export function TagsOverlay({ recording, onClose, onTagsChanged }) {
  const [tags, setTags] = useState(recording.tags || []);
  const [inputValue, setInputValue] = useState('');
  const [allTags, setAllTags] = useState([]);
  const [showSuggestions, setShowSuggestions] = useState(false);
  const [saving, setSaving] = useState(false);
  const inputRef = useRef(null);
  const overlayRef = useRef(null);

  // Load all existing tags for autocomplete
  useEffect(() => {
    recordingsAPI.getAllRecordingTags().then(setAllTags);
  }, []);

  // Close on outside click
  useEffect(() => {
    const handler = (e) => {
      if (overlayRef.current && !overlayRef.current.contains(e.target)) {
        onClose();
      }
    };
    document.addEventListener('mousedown', handler);
    return () => document.removeEventListener('mousedown', handler);
  }, [onClose]);

  // Focus input on open
  useEffect(() => { inputRef.current?.focus(); }, []);

  const suggestions = inputValue.trim()
    ? allTags.filter(t => t.toLowerCase().includes(inputValue.toLowerCase()) && !tags.includes(t))
    : allTags.filter(t => !tags.includes(t));

  const addTag = useCallback(async (tag) => {
    const trimmed = tag.trim();
    if (!trimmed || tags.includes(trimmed)) return;
    const newTags = [...tags, trimmed];
    setTags(newTags);
    setInputValue('');
    setShowSuggestions(false);
    setSaving(true);
    try {
      await recordingsAPI.setRecordingTags(recording.id, newTags);
      if (onTagsChanged) onTagsChanged(recording.id, newTags);
      showStatusMessage(`Tag "${trimmed}" added`);
    } catch { /* error shown by API */ }
    setSaving(false);
  }, [tags, recording.id, onTagsChanged]);

  const removeTag = useCallback(async (tag) => {
    const newTags = tags.filter(t => t !== tag);
    setTags(newTags);
    setSaving(true);
    try {
      await recordingsAPI.setRecordingTags(recording.id, newTags);
      if (onTagsChanged) onTagsChanged(recording.id, newTags);
      showStatusMessage(`Tag "${tag}" removed`);
    } catch { /* error shown by API */ }
    setSaving(false);
  }, [tags, recording.id, onTagsChanged]);

  const handleKeyDown = (e) => {
    if (e.key === 'Enter' && inputValue.trim()) {
      e.preventDefault();
      addTag(inputValue);
    } else if (e.key === 'Escape') {
      onClose();
    }
  };

  return (
    <div ref={overlayRef}
      class="absolute z-50 bg-card border border-border rounded-lg shadow-lg p-3 w-64"
      style={{ minWidth: '240px' }}
    >
      <div class="flex items-center justify-between mb-2">
        <span class="text-sm font-medium flex items-center gap-1">
          <TagIcon className="w-3.5 h-3.5" /> Tags
        </span>
        <button class="text-muted-foreground hover:text-foreground text-xs" onClick={onClose}>✕</button>
      </div>

      {/* Current tags */}
      <div class="flex flex-wrap gap-1 mb-2 min-h-[24px]">
        {tags.length === 0 && <span class="text-xs text-muted-foreground italic">No tags</span>}
        {tags.map(tag => <TagChip key={tag} tag={tag} onRemove={removeTag} />)}
      </div>

      {/* Input with autocomplete */}
      <div class="relative">
        <input
          ref={inputRef}
          type="text"
          class="w-full text-sm px-2 py-1.5 border border-input rounded-md bg-background text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-primary"
          placeholder="Add a tag..."
          value={inputValue}
          onInput={(e) => { setInputValue(e.target.value); setShowSuggestions(true); }}
          onFocus={() => setShowSuggestions(true)}
          onKeyDown={handleKeyDown}
          disabled={saving}
        />
        {showSuggestions && suggestions.length > 0 && (
          <div class="absolute left-0 right-0 mt-1 max-h-32 overflow-y-auto bg-card border border-border rounded-md shadow-lg z-[60]">
            {suggestions.slice(0, 10).map(tag => (
              <button
                key={tag}
                type="button"
                class="w-full text-left px-2 py-1 text-sm hover:bg-muted/70 transition-colors"
                onClick={() => addTag(tag)}
              >
                {tag}
              </button>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

/**
 * Bulk tags overlay for managing tags across multiple selected recordings.
 * Uses tri-state checkboxes:
 *  - checked (all selected recordings have the tag)
 *  - partial (some selected recordings have the tag)
 *  - unchecked (no selected recordings have the tag)
 */
export function BulkTagsOverlay({ recordings, selectedRecordings, onClose, onTagsChanged }) {
  const [allTags, setAllTags] = useState([]);
  const [tagStates, setTagStates] = useState({}); // tag -> 'all' | 'some' | 'none'
  const [inputValue, setInputValue] = useState('');
  const [showSuggestions, setShowSuggestions] = useState(false);
  const [saving, setSaving] = useState(false);
  const overlayRef = useRef(null);
  const inputRef = useRef(null);

  const selectedIds = Object.entries(selectedRecordings)
    .filter(([_, sel]) => sel)
    .map(([id]) => parseInt(id, 10));

  const selectedRecs = recordings.filter(r => selectedIds.includes(r.id));

  // Compute tag states from selected recordings
  useEffect(() => {
    recordingsAPI.getAllRecordingTags().then(setAllTags);
  }, []);

  useEffect(() => {
    const states = {};
    // Gather all tags from selected recordings
    const allUsedTags = new Set();
    selectedRecs.forEach(r => (r.tags || []).forEach(t => allUsedTags.add(t)));
    allTags.forEach(t => allUsedTags.add(t));

    allUsedTags.forEach(tag => {
      const count = selectedRecs.filter(r => (r.tags || []).includes(tag)).length;
      if (count === 0) states[tag] = 'none';
      else if (count === selectedRecs.length) states[tag] = 'all';
      else states[tag] = 'some';
    });
    setTagStates(states);
  }, [allTags, selectedRecs.length]);

  // Close on outside click
  useEffect(() => {
    const handler = (e) => {
      if (overlayRef.current && !overlayRef.current.contains(e.target)) onClose();
    };
    document.addEventListener('mousedown', handler);
    return () => document.removeEventListener('mousedown', handler);
  }, [onClose]);

  const handleTagToggle = useCallback(async (tag) => {
    const current = tagStates[tag] || 'none';
    setSaving(true);
    try {
      if (current === 'all') {
        // Remove from all
        await recordingsAPI.batchUpdateRecordingTags(selectedIds, [], [tag]);
        setTagStates(prev => ({ ...prev, [tag]: 'none' }));
        showStatusMessage(`Tag "${tag}" removed from ${selectedIds.length} recordings`);
      } else {
        // 'none' or 'some' -> add to all
        await recordingsAPI.batchUpdateRecordingTags(selectedIds, [tag], []);
        setTagStates(prev => ({ ...prev, [tag]: 'all' }));
        showStatusMessage(`Tag "${tag}" added to ${selectedIds.length} recordings`);
      }
      if (onTagsChanged) onTagsChanged();
    } catch { /* error shown by API */ }
    setSaving(false);
  }, [tagStates, selectedIds, onTagsChanged]);

  const addNewTag = useCallback(async (tag) => {
    const trimmed = tag.trim();
    if (!trimmed) return;
    setSaving(true);
    try {
      await recordingsAPI.batchUpdateRecordingTags(selectedIds, [trimmed], []);
      setTagStates(prev => ({ ...prev, [trimmed]: 'all' }));
      if (!allTags.includes(trimmed)) setAllTags(prev => [...prev, trimmed].sort());
      setInputValue('');
      setShowSuggestions(false);
      showStatusMessage(`Tag "${trimmed}" added to ${selectedIds.length} recordings`);
      if (onTagsChanged) onTagsChanged();
    } catch { /* error shown by API */ }
    setSaving(false);
  }, [selectedIds, allTags, onTagsChanged]);

  const handleKeyDown = (e) => {
    if (e.key === 'Enter' && inputValue.trim()) { e.preventDefault(); addNewTag(inputValue); }
    else if (e.key === 'Escape') onClose();
  };

  const knownTags = Object.keys(tagStates).sort();
  const suggestions = inputValue.trim()
    ? allTags.filter(t => t.toLowerCase().includes(inputValue.toLowerCase()) && !knownTags.includes(t))
    : [];

  return (
    <div ref={overlayRef}
      class="absolute z-50 bg-card border border-border rounded-lg shadow-lg p-3 w-72"
      style={{ minWidth: '280px' }}
    >
      <div class="flex items-center justify-between mb-2">
        <span class="text-sm font-medium flex items-center gap-1">
          <TagIcon className="w-3.5 h-3.5" /> Manage Tags ({selectedIds.length} recordings)
        </span>
        <button class="text-muted-foreground hover:text-foreground text-xs" onClick={onClose}>✕</button>
      </div>

      {/* Tag list with tri-state checkboxes */}
      <div class="max-h-48 overflow-y-auto mb-2 space-y-0.5">
        {knownTags.length === 0 && <span class="text-xs text-muted-foreground italic">No tags yet</span>}
        {knownTags.map(tag => {
          const state = tagStates[tag];
          return (
            <label key={tag} class="flex items-center gap-2 px-1 py-0.5 rounded hover:bg-muted/50 cursor-pointer text-sm">
              <input
                type="checkbox"
                checked={state === 'all'}
                ref={(el) => { if (el) el.indeterminate = state === 'some'; }}
                onChange={() => handleTagToggle(tag)}
                disabled={saving}
                class="w-4 h-4 rounded"
                style={{ accentColor: 'hsl(var(--primary))' }}
              />
              <span class="truncate">{tag}</span>
            </label>
          );
        })}
      </div>

      {/* Input for new tag */}
      <div class="relative">
        <input
          ref={inputRef}
          type="text"
          class="w-full text-sm px-2 py-1.5 border border-input rounded-md bg-background text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-primary"
          placeholder="Add new tag..."
          value={inputValue}
          onInput={(e) => { setInputValue(e.target.value); setShowSuggestions(true); }}
          onFocus={() => setShowSuggestions(true)}
          onKeyDown={handleKeyDown}
          disabled={saving}
        />
        {showSuggestions && suggestions.length > 0 && (
          <div class="absolute left-0 right-0 mt-1 max-h-32 overflow-y-auto bg-card border border-border rounded-md shadow-lg z-[60]">
            {suggestions.slice(0, 10).map(tag => (
              <button
                key={tag}
                type="button"
                class="w-full text-left px-2 py-1 text-sm hover:bg-muted/70 transition-colors"
                onClick={() => addNewTag(tag)}
              >
                {tag}
              </button>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
