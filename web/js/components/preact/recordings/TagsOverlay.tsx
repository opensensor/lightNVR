/**
 * TagsOverlay component for managing tags on recordings.
 * Shows current tags, allows adding/removing, with autocomplete from existing tags.
 */

import { useState, useEffect, useRef, useCallback } from 'preact/hooks';
import { recordingsAPI } from './recordingsAPI.tsx';
import { showStatusMessage } from '../ToastContainer.tsx';
import { useI18n } from '../../../i18n.ts';
import type { Recording } from '../../../types/core.ts';
import type { RecordingSelection } from '../../../types/recordings.ts';

export interface TagIconProps {
  className?: string;
}

interface TagChipProps {
  tag: string;
  onRemove?: (tag: string) => unknown;
  readOnly?: boolean;
  disabled?: boolean;
}

export interface TagsOverlayProps<Id extends number | string = number | string> {
  /** Only id and tags are read, so TimelineControls can pass a partial recording. */
  recording: { id: Id; tags?: string[] };
  onClose: () => void;
  onTagsChanged?: (recordingId: Id, newTags: string[]) => unknown;
}

export interface BulkTagsOverlayProps {
  recordings: Array<Pick<Recording, 'id'> & { tags?: string[] }>;
  selectedRecordings: RecordingSelection;
  onClose: () => void;
  onTagsChanged?: () => unknown;
}

type BulkTagState = 'all' | 'some' | 'none';

/**
 * Tag icon SVG (label/tag shape)
 */
export function TagIcon({ className = 'w-4 h-4' }: TagIconProps) {
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
function TagChip({ tag, onRemove, readOnly = false, disabled = false }: TagChipProps) {
  return (
    <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs bg-primary/10 text-primary border border-primary/20">
      {tag}
      {!readOnly && onRemove && (
        <button
          type="button"
          class="hover:text-destructive ms-0.5"
          onClick={(e) => { e.stopPropagation(); onRemove(tag); }}
          title={`Remove tag "${tag}"`}
          disabled={disabled}
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
export function TagsOverlay<Id extends number | string>({ recording, onClose, onTagsChanged }: TagsOverlayProps<Id>) {
  const { t } = useI18n();
  const [tags, setTags] = useState<string[]>(recording.tags || []);
  const [inputValue, setInputValue] = useState('');
  const [allTags, setAllTags] = useState<string[]>([]);
  const [showSuggestions, setShowSuggestions] = useState(false);
  const [saving, setSaving] = useState(false);
  const savingRef = useRef(false);
  const inputRef = useRef<HTMLInputElement | null>(null);
  const overlayRef = useRef<HTMLDivElement | null>(null);

  // Load all existing tags for autocomplete
  useEffect(() => {
    recordingsAPI.getAllRecordingTags().then(setAllTags);
  }, []);

  // Close on outside click
  useEffect(() => {
    const handler = (e: MouseEvent) => {
      if (overlayRef.current && !overlayRef.current.contains(e.target as Node | null)) {
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

  const addTag = useCallback(async (tag: string) => {
    const trimmed = tag.trim();
    if (savingRef.current || !trimmed || tags.includes(trimmed)) return;
    savingRef.current = true;
    const previousTags = tags;
    const newTags = [...tags, trimmed];
    setTags(newTags);
    setInputValue('');
    setShowSuggestions(false);
    setSaving(true);
    try {
      await recordingsAPI.setRecordingTags(recording.id, newTags);
    } catch {
      // The API reports the error; restore the last confirmed tag set.
      setTags(previousTags);
      setInputValue(trimmed);
      return;
    } finally {
      savingRef.current = false;
      setSaving(false);
    }
    if (onTagsChanged) onTagsChanged(recording.id, newTags);
    showStatusMessage(t('recordings.tagAdded', { tag: trimmed }));
  }, [tags, recording.id, onTagsChanged, t]);

  const removeTag = useCallback(async (tag: string) => {
    if (savingRef.current || !tags.includes(tag)) return;
    savingRef.current = true;
    const previousTags = tags;
    const newTags = tags.filter(t => t !== tag);
    setTags(newTags);
    setSaving(true);
    try {
      await recordingsAPI.setRecordingTags(recording.id, newTags);
    } catch {
      // The API reports the error; restore the last confirmed tag set.
      setTags(previousTags);
      return;
    } finally {
      savingRef.current = false;
      setSaving(false);
    }
    if (onTagsChanged) onTagsChanged(recording.id, newTags);
    showStatusMessage(t('recordings.tagRemoved', { tag }));
  }, [tags, recording.id, onTagsChanged, t]);

  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === 'Enter' && inputValue.trim()) {
      e.preventDefault();
      addTag(inputValue);
    } else if (e.key === 'Escape') {
      onClose();
    }
  };

  return (
    <div ref={overlayRef}
      data-keyboard-nav-preserve
      class="absolute z-50 bg-card border border-border rounded-lg shadow-lg p-3 w-64"
      style={{ minWidth: '240px' }}
    >
      <div class="flex items-center justify-between mb-2">
        <span class="text-sm font-medium flex items-center gap-1">
          <TagIcon className="w-3.5 h-3.5" /> {t('recordings.recordingTags')}
        </span>
        <button class="text-muted-foreground hover:text-foreground text-xs" onClick={onClose}>✕</button>
      </div>

      {/* Current tags */}
      <div class="flex flex-wrap gap-1 mb-2 min-h-[24px]">
        {tags.length === 0 && <span class="text-xs text-muted-foreground italic">{t('recordings.noTags')}</span>}
        {tags.map(tag => <TagChip key={tag} tag={tag} onRemove={removeTag} disabled={saving} />)}
      </div>

      {/* Input with autocomplete */}
      <div class="relative">
        <input
          ref={inputRef}
          type="text"
          class="w-full text-sm px-2 py-1.5 border border-input rounded-md bg-background text-foreground placeholder:text-muted-foreground focus:ring-1 focus:ring-primary"
          placeholder={t('recordings.addTagPlaceholder')}
          value={inputValue}
          onInput={(e) => { setInputValue((e.target as HTMLInputElement).value); setShowSuggestions(true); }}
          onFocus={() => setShowSuggestions(true)}
          onKeyDown={handleKeyDown}
          disabled={saving}
        />
        {showSuggestions && suggestions.length > 0 && (
          <div class="absolute inset-s-0 inset-e-0 mt-1 max-h-32 overflow-y-auto bg-card border border-border rounded-md shadow-lg z-[60]">
            {suggestions.slice(0, 10).map(tag => (
              <button
                key={tag}
                type="button"
                class="w-full text-start px-2 py-1 text-sm hover:bg-muted/70 transition-colors"
                disabled={saving}
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
export function BulkTagsOverlay({ recordings, selectedRecordings, onClose, onTagsChanged }: BulkTagsOverlayProps) {
  const { t } = useI18n();
  const [allTags, setAllTags] = useState<string[]>([]);
  const [tagStates, setTagStates] = useState<Record<string, BulkTagState>>({}); // tag -> 'all' | 'some' | 'none'
  const [inputValue, setInputValue] = useState('');
  const [showSuggestions, setShowSuggestions] = useState(false);
  const [saving, setSaving] = useState(false);
  const overlayRef = useRef<HTMLDivElement | null>(null);
  const inputRef = useRef<HTMLInputElement | null>(null);

  const selectedIds = Object.entries(selectedRecordings)
    .filter(([_, sel]) => sel)
    .map(([id]) => parseInt(id, 10));

  const selectionKey = JSON.stringify(selectedIds);
  const currentSelectionRef = useRef(selectionKey);
  currentSelectionRef.current = selectionKey;

  // The catalog is independent of the recording snapshot. Loading more names
  // must not overwrite a successful local edit while the parent refetches rows.
  useEffect(() => {
    recordingsAPI.getAllRecordingTags().then(setAllTags);
  }, []);

  useEffect(() => {
    setTagStates(prev => {
      const states = { ...prev };
      allTags.forEach(tag => {
        if (!Object.hasOwn(states, tag)) states[tag] = 'none';
      });
      return states;
    });
  }, [allTags]);

  // A new recording snapshot is authoritative, including same-count refetches.
  // Use selected IDs rather than their count so replacing a selection reconciles
  // correctly without rerunning on each newly allocated selection object.
  useEffect(() => {
    const ids: number[] = JSON.parse(selectionKey);
    const selectedRecs = recordings.filter(recording => ids.includes(recording.id));
    setTagStates(prev => {
      const states: Record<string, BulkTagState> = {};
      const knownTags = new Set<string>(Object.keys(prev));
      selectedRecs.forEach(recording => (recording.tags || []).forEach(tag => knownTags.add(tag)));
      knownTags.forEach(tag => {
        const count = selectedRecs.filter(recording => (recording.tags || []).includes(tag)).length;
        states[tag] = count === 0 ? 'none' : count === selectedRecs.length ? 'all' : 'some';
      });
      return states;
    });
  }, [recordings, selectionKey]);

  // Close on outside click
  useEffect(() => {
    const handler = (e: MouseEvent) => {
      if (overlayRef.current && !overlayRef.current.contains(e.target as Node | null)) onClose();
    };
    document.addEventListener('mousedown', handler);
    return () => document.removeEventListener('mousedown', handler);
  }, [onClose]);

  const handleTagToggle = useCallback(async (tag: string) => {
    const current = tagStates[tag] || 'none';
    setSaving(true);
    try {
      if (current === 'all') {
        // Remove from all
        await recordingsAPI.batchUpdateRecordingTags(selectedIds, [], [tag]);
        if (currentSelectionRef.current === selectionKey) {
          setTagStates(prev => ({ ...prev, [tag]: 'none' }));
        }
        showStatusMessage(t('recordings.tagRemovedFromRecordings', { tag, count: selectedIds.length }));
      } else {
        // 'none' or 'some' -> add to all
        await recordingsAPI.batchUpdateRecordingTags(selectedIds, [tag], []);
        if (currentSelectionRef.current === selectionKey) {
          setTagStates(prev => ({ ...prev, [tag]: 'all' }));
        }
        showStatusMessage(t('recordings.tagAddedToRecordings', { tag, count: selectedIds.length }));
      }
      if (onTagsChanged) onTagsChanged();
    } catch { /* error shown by API */ }
    setSaving(false);
  }, [tagStates, selectedIds, selectionKey, onTagsChanged]);

  const addNewTag = useCallback(async (tag: string) => {
    const trimmed = tag.trim();
    if (!trimmed) return;
    setSaving(true);
    try {
      await recordingsAPI.batchUpdateRecordingTags(selectedIds, [trimmed], []);
      if (currentSelectionRef.current === selectionKey) {
        setTagStates(prev => ({ ...prev, [trimmed]: 'all' }));
      }
      if (!allTags.includes(trimmed)) setAllTags(prev => [...prev, trimmed].sort());
      setInputValue('');
      setShowSuggestions(false);
      showStatusMessage(t('recordings.tagAddedToRecordings', { tag: trimmed, count: selectedIds.length }));
      if (onTagsChanged) onTagsChanged();
    } catch { /* error shown by API */ }
    setSaving(false);
  }, [selectedIds, selectionKey, allTags, onTagsChanged]);

  const handleKeyDown = (e: KeyboardEvent) => {
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
          <TagIcon className="w-3.5 h-3.5" /> {t('recordings.recordingTagsCount', { count: selectedIds.length })}
        </span>
        <button class="text-muted-foreground hover:text-foreground text-xs" onClick={onClose}>✕</button>
      </div>

      {/* Tag list with tri-state checkboxes */}
      <div class="max-h-48 overflow-y-auto mb-2 space-y-0.5">
        {knownTags.length === 0 && <span class="text-xs text-muted-foreground italic">{t('recordings.noTagsYet')}</span>}
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
          class="w-full text-sm px-2 py-1.5 border border-input rounded-md bg-background text-foreground placeholder:text-muted-foreground focus:ring-1 focus:ring-primary"
          placeholder={t('recordings.addNewTagPlaceholder')}
          value={inputValue}
          onInput={(e) => { setInputValue((e.target as HTMLInputElement).value); setShowSuggestions(true); }}
          onFocus={() => setShowSuggestions(true)}
          onKeyDown={handleKeyDown}
          disabled={saving}
        />
        {showSuggestions && suggestions.length > 0 && (
          <div class="absolute inset-s-0 inset-e-0 mt-1 max-h-32 overflow-y-auto bg-card border border-border rounded-md shadow-lg z-[60]">
            {suggestions.slice(0, 10).map(tag => (
              <button
                key={tag}
                type="button"
                class="w-full text-start px-2 py-1 text-sm hover:bg-muted/70 transition-colors"
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
