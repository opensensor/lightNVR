/**
 * Stream Configuration Modal Component
 * Expanded, responsive modal with accordion sections for stream configuration
 */

import { useState } from 'react';

/**
 * Accordion Section Component
 */
function AccordionSection({ title, isExpanded, onToggle, children, badge }) {
  return (
    <div className="border border-border rounded-lg mb-3">
      <button
        type="button"
        onClick={onToggle}
        className="w-full flex items-center justify-between p-4 text-left hover:bg-muted/50 transition-colors rounded-t-lg"
      >
        <div className="flex items-center space-x-2">
          <h4 className="text-md font-semibold text-foreground">{title}</h4>
          {badge && (
            <span className="px-2 py-0.5 text-xs rounded-full bg-primary/10 text-primary">
              {badge}
            </span>
          )}
        </div>
        <svg
          className={`w-5 h-5 transition-transform ${isExpanded ? 'transform rotate-180' : ''}`}
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
        </svg>
      </button>
      {isExpanded && (
        <div className="p-4 pt-3 space-y-4 border-t border-border">
          {children}
        </div>
      )}
    </div>
  );
}

/**
 * Main Stream Configuration Modal
 */
export function StreamConfigModal({
  isEditing,
  currentStream,
  detectionModels,
  expandedSections,
  onToggleSection,
  onInputChange,
  onThresholdChange,
  onTestConnection,
  onTestMotion,
  onSave,
  onClose,
  onRefreshModels
}) {
  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div className="bg-card text-card-foreground rounded-lg shadow-xl w-full max-w-5xl max-h-[95vh] overflow-hidden flex flex-col">
        {/* Header */}
        <div className="flex justify-between items-center p-6 border-b border-border flex-shrink-0">
          <div>
            <h3 className="text-2xl font-bold">{isEditing ? 'Edit Stream' : 'Add Stream'}</h3>
            <p className="text-sm text-muted-foreground mt-1">
              Configure stream settings, recording options, and detection parameters
            </p>
          </div>
          <button
            onClick={onClose}
            className="text-muted-foreground hover:text-foreground transition-colors p-2 rounded-full hover:bg-muted"
          >
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        {/* Scrollable Content */}
        <div className="flex-1 overflow-y-auto p-6">
          <form id="stream-form" className="space-y-3">

            {/* Basic Settings Section */}
            <AccordionSection
              title="Basic Settings"
              isExpanded={expandedSections.basic}
              onToggle={() => onToggleSection('basic')}
            >
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div className="md:col-span-2">
                  <label htmlFor="stream-name" className="block text-sm font-medium mb-2">
                    Stream Name <span className="text-danger">*</span>
                  </label>
                  <input
                    type="text"
                    id="stream-name"
                    name="name"
                    className={`w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground ${isEditing ? 'bg-muted/30' : ''}`}
                    value={currentStream.name}
                    onChange={onInputChange}
                    disabled={isEditing}
                    required
                    placeholder="e.g., front_door, backyard"
                  />
                  {isEditing && (
                    <p className="mt-1 text-xs text-muted-foreground">Stream name cannot be changed after creation</p>
                  )}
                </div>

                <div className="md:col-span-2">
                  <label htmlFor="stream-url" className="block text-sm font-medium mb-2">
                    Stream URL <span className="text-danger">*</span>
                  </label>
                  <input
                    type="text"
                    id="stream-url"
                    name="url"
                    className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                    placeholder="rtsp://username:password@192.168.1.100:554/stream"
                    value={currentStream.url}
                    onChange={onInputChange}
                    required
                  />
                  <p className="mt-1 text-xs text-muted-foreground">RTSP URL for the camera stream</p>
                </div>

                <div className="flex items-center space-x-6">
                  <label className="flex items-center space-x-2 cursor-pointer">
                    <input
                      type="checkbox"
                      id="stream-enabled"
                      name="enabled"
                      className="h-4 w-4 rounded border-gray-300"
                      style={{accentColor: 'hsl(var(--primary))'}}
                      checked={currentStream.enabled}
                      onChange={onInputChange}
                    />
                    <span className="text-sm font-medium">Stream Active</span>
                  </label>
                  <label className="flex items-center space-x-2 cursor-pointer">
                    <input
                      type="checkbox"
                      id="stream-streaming-enabled"
                      name="streamingEnabled"
                      className="h-4 w-4 rounded border-gray-300"
                      style={{accentColor: 'hsl(var(--primary))'}}
                      checked={currentStream.streamingEnabled}
                      onChange={onInputChange}
                    />
                    <span className="text-sm font-medium">Live View Enabled</span>
                  </label>
                </div>

                <div className="flex items-center">
                  <label className="flex items-center space-x-2 cursor-pointer">
                    <input
                      type="checkbox"
                      id="stream-is-onvif"
                      name="isOnvif"
                      className="h-4 w-4 rounded border-gray-300"
                      style={{accentColor: 'hsl(var(--primary))'}}
                      checked={currentStream.isOnvif}
                      onChange={onInputChange}
                    />
                    <span className="text-sm font-medium">ONVIF Camera</span>
                  </label>
                  <span className="ml-2 text-xs text-muted-foreground">Enables motion detection features</span>
                </div>

                <div>
                  <label htmlFor="stream-width" className="block text-sm font-medium mb-2">Width</label>
                  <input
                    type="number"
                    id="stream-width"
                    name="width"
                    className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                    min="320"
                    max="3840"
                    value={currentStream.width}
                    onChange={onInputChange}
                  />
                </div>

                <div>
                  <label htmlFor="stream-height" className="block text-sm font-medium mb-2">Height</label>
                  <input
                    type="number"
                    id="stream-height"
                    name="height"
                    className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                    min="240"
                    max="2160"
                    value={currentStream.height}
                    onChange={onInputChange}
                  />
                </div>

                <div>
                  <label htmlFor="stream-fps" className="block text-sm font-medium mb-2">FPS</label>
                  <input
                    type="number"
                    id="stream-fps"
                    name="fps"
                    className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                    min="1"
                    max="60"
                    value={currentStream.fps}
                    onChange={onInputChange}
                  />
                </div>

                <div>
                  <label htmlFor="stream-codec" className="block text-sm font-medium mb-2">Codec</label>
                  <select
                    id="stream-codec"
                    name="codec"
                    className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                    value={currentStream.codec}
                    onChange={onInputChange}
                  >
                    <option value="h264">H.264</option>
                    <option value="h265">H.265 (HEVC)</option>
                  </select>
                </div>

                <div>
                  <label htmlFor="stream-protocol" className="block text-sm font-medium mb-2">Protocol</label>
                  <select
                    id="stream-protocol"
                    name="protocol"
                    className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                    value={currentStream.protocol}
                    onChange={onInputChange}
                  >
                    <option value="0">TCP</option>
                    <option value="1">UDP</option>
                  </select>
                </div>
              </div>
            </AccordionSection>

            {/* Recording Settings Section */}
            <AccordionSection
              title="Recording Settings"
              isExpanded={expandedSections.recording}
              onToggle={() => onToggleSection('recording')}
            >
              <div className="space-y-4">
                <div className="flex items-center space-x-4">
                  <label className="flex items-center space-x-2 cursor-pointer">
                    <input
                      type="checkbox"
                      id="stream-record"
                      name="record"
                      className="h-4 w-4 rounded border-gray-300"
                      style={{accentColor: 'hsl(var(--primary))'}}
                      checked={currentStream.record}
                      onChange={onInputChange}
                    />
                    <span className="text-sm font-medium">Enable Continuous Recording</span>
                  </label>
                  <label className="flex items-center space-x-2 cursor-pointer">
                    <input
                      type="checkbox"
                      id="stream-record-audio"
                      name="recordAudio"
                      className="h-4 w-4 rounded border-gray-300"
                      style={{accentColor: 'hsl(var(--primary))'}}
                      checked={currentStream.recordAudio}
                      onChange={onInputChange}
                    />
                    <span className="text-sm font-medium">Record Audio</span>
                  </label>
                </div>
                <p className="text-sm text-muted-foreground">
                  Continuous recording saves all video to disk. Audio recording requires the stream to have an audio track.
                </p>
              </div>
            </AccordionSection>

            {/* AI Detection Recording Section */}
            <AccordionSection
              title="AI Detection Recording"
              isExpanded={expandedSections.detection}
              onToggle={() => onToggleSection('detection')}
              badge="Optional"
            >
              <div className="space-y-4">
                <div className="flex items-center">
                  <label className="flex items-center space-x-2 cursor-pointer">
                    <input
                      type="checkbox"
                      id="stream-detection-enabled"
                      name="detectionEnabled"
                      className="h-4 w-4 rounded border-gray-300"
                      style={{accentColor: 'hsl(var(--primary))'}}
                      checked={currentStream.detectionEnabled}
                      onChange={onInputChange}
                    />
                    <span className="text-sm font-medium">Enable AI Detection-Based Recording</span>
                  </label>
                </div>
                <p className="text-sm text-muted-foreground">
                  Only record when AI detects specific objects (person, car, etc.). Requires a detection model.
                </p>

                {currentStream.detectionEnabled && (
                  <>
                    <div>
                      <label htmlFor="stream-detection-model" className="block text-sm font-medium mb-2">
                        Detection Model
                      </label>
                      <div className="flex space-x-2">
                        <select
                          id="stream-detection-model"
                          name="detectionModel"
                          className="flex-1 px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                          value={currentStream.detectionModel}
                          onChange={onInputChange}
                        >
                          <option value="">Select a model</option>
                          {detectionModels.map(model => (
                            <option key={model.id} value={model.id}>{model.name}</option>
                          ))}
                        </select>
                        <button
                          type="button"
                          onClick={onRefreshModels}
                          className="p-2 rounded-md bg-secondary hover:bg-secondary/80 text-secondary-foreground focus:outline-none"
                          title="Refresh Models"
                        >
                          <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                            <path fillRule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clipRule="evenodd" />
                          </svg>
                        </button>
                      </div>
                    </div>

                    <div>
                      <label htmlFor="stream-detection-threshold" className="block text-sm font-medium mb-2">
                        Detection Threshold: <span className="text-primary font-semibold">{currentStream.detectionThreshold}%</span>
                      </label>
                      <input
                        type="range"
                        id="stream-detection-threshold"
                        name="detectionThreshold"
                        className="w-full h-2 bg-muted rounded-lg appearance-none cursor-pointer"
                        min="0"
                        max="100"
                        step="1"
                        value={currentStream.detectionThreshold}
                        onInput={onThresholdChange}
                      />
                      <p className="mt-1 text-xs text-muted-foreground">
                        Minimum confidence level to trigger recording (0-100%)
                      </p>
                    </div>

                    <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                      <div>
                        <label htmlFor="stream-detection-interval" className="block text-sm font-medium mb-2">
                          Detection Interval (frames)
                        </label>
                        <input
                          type="number"
                          id="stream-detection-interval"
                          name="detectionInterval"
                          className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                          min="1"
                          max="100"
                          value={currentStream.detectionInterval}
                          onChange={onInputChange}
                        />
                        <p className="mt-1 text-xs text-muted-foreground">Run detection every N frames</p>
                      </div>

                      <div>
                        <label htmlFor="stream-pre-buffer" className="block text-sm font-medium mb-2">
                          Pre-Detection Buffer (sec)
                        </label>
                        <input
                          type="number"
                          id="stream-pre-buffer"
                          name="preBuffer"
                          className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                          min="0"
                          max="60"
                          value={currentStream.preBuffer}
                          onChange={onInputChange}
                        />
                        <p className="mt-1 text-xs text-muted-foreground">Seconds before detection</p>
                      </div>

                      <div>
                        <label htmlFor="stream-post-buffer" className="block text-sm font-medium mb-2">
                          Post-Detection Buffer (sec)
                        </label>
                        <input
                          type="number"
                          id="stream-post-buffer"
                          name="postBuffer"
                          className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                          min="0"
                          max="300"
                          value={currentStream.postBuffer}
                          onChange={onInputChange}
                        />
                        <p className="mt-1 text-xs text-muted-foreground">Seconds after detection</p>
                      </div>
                    </div>
                  </>
                )}
              </div>
            </AccordionSection>

            {/* Motion Recording Section (ONVIF only) */}
            {currentStream.isOnvif && (
              <AccordionSection
                title="Motion Recording (ONVIF)"
                isExpanded={expandedSections.motion}
                onToggle={() => onToggleSection('motion')}
                badge="ONVIF Only"
              >
                <div className="space-y-4">
                  <div className="flex items-center">
                    <label className="flex items-center space-x-2 cursor-pointer">
                      <input
                        type="checkbox"
                        id="motion-recording-enabled"
                        name="motionRecordingEnabled"
                        className="h-4 w-4 rounded border-gray-300"
                        style={{accentColor: 'hsl(var(--primary))'}}
                        checked={currentStream.motionRecordingEnabled}
                        onChange={onInputChange}
                      />
                      <span className="text-sm font-medium">Enable ONVIF Motion Recording</span>
                    </label>
                  </div>
                  <p className="text-sm text-muted-foreground">
                    Record when the camera's built-in motion detection triggers. Uses ONVIF events from the camera.
                  </p>

                  {currentStream.motionRecordingEnabled && (
                    <>
                      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div>
                          <label htmlFor="motion-pre-buffer" className="block text-sm font-medium mb-2">
                            Pre-Event Buffer (seconds)
                          </label>
                          <input
                            type="number"
                            id="motion-pre-buffer"
                            name="motionPreBuffer"
                            className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                            min="0"
                            max="30"
                            value={currentStream.motionPreBuffer}
                            onChange={onInputChange}
                          />
                          <p className="mt-1 text-xs text-muted-foreground">Video before motion event (0-30s)</p>
                        </div>

                        <div>
                          <label htmlFor="motion-post-buffer" className="block text-sm font-medium mb-2">
                            Post-Event Buffer (seconds)
                          </label>
                          <input
                            type="number"
                            id="motion-post-buffer"
                            name="motionPostBuffer"
                            className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                            min="0"
                            max="60"
                            value={currentStream.motionPostBuffer}
                            onChange={onInputChange}
                          />
                          <p className="mt-1 text-xs text-muted-foreground">Continue recording after motion (0-60s)</p>
                        </div>

                        <div>
                          <label htmlFor="motion-max-duration" className="block text-sm font-medium mb-2">
                            Max File Duration (seconds)
                          </label>
                          <input
                            type="number"
                            id="motion-max-duration"
                            name="motionMaxDuration"
                            className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                            min="60"
                            max="3600"
                            step="60"
                            value={currentStream.motionMaxDuration}
                            onChange={onInputChange}
                          />
                          <p className="mt-1 text-xs text-muted-foreground">Max duration per file (60-3600s)</p>
                        </div>

                        <div>
                          <label htmlFor="motion-retention-days" className="block text-sm font-medium mb-2">
                            Retention Period (days)
                          </label>
                          <input
                            type="number"
                            id="motion-retention-days"
                            name="motionRetentionDays"
                            className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                            min="1"
                            max="365"
                            value={currentStream.motionRetentionDays}
                            onChange={onInputChange}
                          />
                          <p className="mt-1 text-xs text-muted-foreground">Auto-delete after N days (1-365)</p>
                        </div>

                        <div>
                          <label htmlFor="motion-codec" className="block text-sm font-medium mb-2">
                            Video Codec
                          </label>
                          <select
                            id="motion-codec"
                            name="motionCodec"
                            className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                            value={currentStream.motionCodec}
                            onChange={onInputChange}
                          >
                            <option value="h264">H.264</option>
                            <option value="h265">H.265 (HEVC)</option>
                          </select>
                        </div>

                        <div>
                          <label htmlFor="motion-quality" className="block text-sm font-medium mb-2">
                            Recording Quality
                          </label>
                          <select
                            id="motion-quality"
                            name="motionQuality"
                            className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                            value={currentStream.motionQuality}
                            onChange={onInputChange}
                          >
                            <option value="low">Low</option>
                            <option value="medium">Medium</option>
                            <option value="high">High</option>
                          </select>
                        </div>

	                  <div className="flex justify-end pt-2">
	                    <button
	                      type="button"
	                      onClick={onTestMotion}
	                      className="px-4 py-2 bg-secondary text-secondary-foreground rounded-md hover:bg-secondary/80 transition-colors font-medium"
	                    >
	                      Trigger Test Motion Event
	                    </button>
	                  </div>

                      </div>
                    </>
                  )}
                </div>
              </AccordionSection>
            )}

            {/* Advanced Settings Section */}
            <AccordionSection
              title="Advanced Settings"
              isExpanded={expandedSections.advanced}
              onToggle={() => onToggleSection('advanced')}
            >
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label htmlFor="stream-priority" className="block text-sm font-medium mb-2">
                    Stream Priority
                  </label>
                  <select
                    id="stream-priority"
                    name="priority"
                    className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                    value={currentStream.priority}
                    onChange={onInputChange}
                  >
                    <option value="1">Low (1)</option>
                    <option value="5">Medium (5)</option>
                    <option value="10">High (10)</option>
                  </select>
                  <p className="mt-1 text-xs text-muted-foreground">Processing priority for this stream</p>
                </div>

                <div>
                  <label htmlFor="stream-segment" className="block text-sm font-medium mb-2">
                    Segment Duration (seconds)
                  </label>
                  <input
                    type="number"
                    id="stream-segment"
                    name="segment"
                    className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                    min="60"
                    max="3600"
                    value={currentStream.segment}
                    onChange={onInputChange}
                  />
                  <p className="mt-1 text-xs text-muted-foreground">Duration of each recording segment</p>
                </div>
              </div>
            </AccordionSection>

          </form>
        </div>

        {/* Footer */}
        <div className="flex justify-between items-center p-6 border-t border-border flex-shrink-0 bg-muted/20">
          <button
            type="button"
            onClick={onTestConnection}
            className="px-4 py-2 bg-secondary text-secondary-foreground rounded-md hover:bg-secondary/80 transition-colors font-medium"
          >
            Test Connection
          </button>
          <div className="flex space-x-3">
            <button
              type="button"
              onClick={onClose}
              className="px-6 py-2 border border-input rounded-md shadow-sm text-sm font-medium text-foreground bg-background hover:bg-muted transition-colors"
            >
              Cancel
            </button>
            <button
              type="button"
              onClick={onSave}
              className="px-6 py-2 btn-primary font-medium"
            >
              {isEditing ? 'Update Stream' : 'Add Stream'}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}

