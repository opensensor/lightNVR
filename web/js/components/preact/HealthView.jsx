/**
 * HealthView - Stream Health monitoring page
 * Shows fleet summary, per-stream health cards with sparklines
 */
import { useState } from 'preact/hooks';
import { useQuery } from '../../query-client.js';
import { useI18n } from '../../i18n.js';
import { Sparkline } from './health/Sparkline.jsx';

function StatusBadge({ status }) {
    const colors = {
        up:       { bg: 'hsl(142 70% 45% / 0.15)', text: 'hsl(142 70% 35%)' },
        degraded: { bg: 'hsl(45 93% 47% / 0.15)',   text: 'hsl(45 80% 35%)' },
        down:     { bg: 'hsl(0 84% 60% / 0.15)',    text: 'hsl(0 84% 40%)' },
    };
    const c = colors[status] || colors.down;
    return (
        <span class="inline-block px-2 py-0.5 text-xs font-semibold rounded-full"
              style={{ backgroundColor: c.bg, color: c.text }}>
            {status}
        </span>
    );
}

function OverallStatusBadge({ status }) {
    const colors = {
        healthy:  { bg: 'hsl(142 70% 45% / 0.15)', text: 'hsl(142 70% 35%)' },
        degraded: { bg: 'hsl(45 93% 47% / 0.15)',   text: 'hsl(45 80% 35%)' },
        critical: { bg: 'hsl(0 84% 60% / 0.15)',    text: 'hsl(0 84% 40%)' },
    };
    const c = colors[status] || colors.critical;
    return (
        <span class="inline-block px-3 py-1 text-sm font-semibold rounded-full"
              style={{ backgroundColor: c.bg, color: c.text }}>
            {status}
        </span>
    );
}

function formatBitrate(bps) {
    if (!bps || bps <= 0) return '0 bps';
    if (bps >= 1000000) return (bps / 1000000).toFixed(1) + ' Mbps';
    if (bps >= 1000) return (bps / 1000).toFixed(0) + ' Kbps';
    return bps.toFixed(0) + ' bps';
}

function formatBytes(bytes) {
    if (!bytes || bytes <= 0) return '0 B';
    if (bytes >= 1073741824) return (bytes / 1073741824).toFixed(1) + ' GB';
    if (bytes >= 1048576) return (bytes / 1048576).toFixed(1) + ' MB';
    return (bytes / 1024).toFixed(0) + ' KB';
}

function formatDuration(seconds) {
    if (!seconds || seconds <= 0) return '0s';
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    const s = Math.floor(seconds % 60);
    if (h > 0) return `${h}h ${m}m`;
    if (m > 0) return `${m}m ${s}s`;
    return `${s}s`;
}

function formatTimeAgo(ts) {
    if (!ts || ts <= 0) return 'never';
    const ago = Math.floor(Date.now() / 1000) - ts;
    if (ago < 5) return 'just now';
    if (ago < 60) return `${ago}s ago`;
    if (ago < 3600) return `${Math.floor(ago / 60)}m ago`;
    return `${Math.floor(ago / 3600)}h ago`;
}

function StreamCard({ stream, expanded, onToggle }) {
    const { t } = useI18n();
    const observedFps = stream.observed_fps ?? stream.fps ?? 0;
    const expectedFps = stream.expected_fps ?? stream.configured_fps ?? 30;

    return (
        <div class="bg-card text-card-foreground rounded-lg shadow p-4 cursor-pointer hover:shadow-md transition-shadow"
             onClick={onToggle}>
            {/* Header */}
            <div class="flex items-center justify-between mb-2">
                <h3 class="text-sm font-semibold truncate mr-2">{stream.name}</h3>
                <StatusBadge status={stream.status} />
            </div>

            {/* Key metrics */}
            <div class="grid grid-cols-2 gap-x-4 gap-y-1 text-xs text-muted-foreground">
                <div>
                    <span class="opacity-60" title={t('streamHealth.fpsHelp')}>
                        {t('streamHealth.observedFps')}:
                    </span>{' '}
                    <span class="text-foreground font-medium">
                        {observedFps.toFixed(1)}
                    </span>
                    <div class="opacity-60 mt-0.5">
                        {t('streamHealth.expectedFps')}: {expectedFps} {t('streamHealth.fps')}
                    </div>
                </div>
                <div>
                    <span class="opacity-60">{t('streamHealth.bitrate')}:</span>{' '}
                    <span class="text-foreground font-medium">{formatBitrate(stream.bitrate_bps)}</span>
                </div>
                <div>
                    <span class="opacity-60">{t('streamHealth.uptime')}:</span>{' '}
                    <span class="text-foreground font-medium">{formatDuration(stream.uptime_seconds)}</span>
                </div>
                <div>
                    <span class="opacity-60">{t('streamHealth.lastFrame')}:</span>{' '}
                    <span class="text-foreground font-medium">{formatTimeAgo(stream.last_frame_ts)}</span>
                </div>
            </div>

            {/* Sparklines */}
            <div class="mt-3 flex gap-4">
                <div class="flex-1">
                    <div class="text-xs text-muted-foreground opacity-60 mb-0.5">{t('streamHealth.fps')}</div>
                    <Sparkline data={stream.sparkline_fps} width={140} height={24}
                               color="hsl(var(--primary))" />
                </div>
                <div class="flex-1">
                    <div class="text-xs text-muted-foreground opacity-60 mb-0.5">{t('streamHealth.bitrate')}</div>
                    <Sparkline data={stream.sparkline_bitrate} width={140} height={24}
                               color="hsl(var(--info))" />
                </div>
            </div>

            {/* Expanded detail */}
            {expanded && (
                <div class="mt-3 pt-3 border-t border-border text-xs text-muted-foreground">
                    <div class="grid grid-cols-2 gap-x-4 gap-y-1">
                        <div>{t('streamHealth.framesTotal')}: <span class="text-foreground">{stream.frames_total?.toLocaleString() || 0}</span></div>
                        <div>{t('streamHealth.framesDropped')}: <span class="text-foreground">{stream.frames_dropped?.toLocaleString() || 0}</span></div>
                        <div>{t('streamHealth.reconnects')}: <span class="text-foreground">{stream.reconnects || 0}</span></div>
                        <div>{t('streamHealth.connectionLatency')}: <span class="text-foreground">{stream.connection_latency_ms?.toFixed(0) || 0}ms</span></div>
                        <div>{t('streamHealth.recording')}: <span class="text-foreground">{stream.recording_active ? t('streamHealth.active') : t('streamHealth.inactive')}</span></div>
                        <div>{t('streamHealth.recordingGaps')}: <span class="text-foreground">{stream.recording_gaps || 0}</span></div>
                    </div>
                </div>
            )}
        </div>
    );
}

export function HealthView() {
    const { t } = useI18n();
    const [expandedStream, setExpandedStream] = useState(null);

    const { data: health, isLoading, error } = useQuery(
        ['streamHealth'],
        '/api/health?sparklines=true',
        { timeout: 10000, retries: 1 },
        { refetchInterval: 5000 }
    );

    if (isLoading && !health) {
        return (
            <div>
                <div class="text-muted-foreground">{t('streamHealth.loading')}</div>
            </div>
        );
    }

    if (error && !health) {
        return (
            <div>
                <div class="text-destructive">{t('streamHealth.error')}</div>
            </div>
        );
    }

    const streams = health?.streams || { total: 0, up: 0, degraded: 0, down: 0 };
    const streamsDetail = health?.streams_detail || [];
    const storage = health?.storage || {};
    const overallStatus = health?.status || 'healthy';
    const storagePercent = storage.used_bytes && storage.available_bytes
        ? ((storage.used_bytes / (storage.used_bytes + storage.available_bytes)) * 100).toFixed(1)
        : 0;

    return (
        <div>
            {/* Fleet Summary Bar */}
            <div class="bg-card text-card-foreground rounded-lg shadow p-4 mb-6">
                <div class="flex flex-wrap items-center gap-6">
                    <div class="flex items-center gap-2">
                        <span class="text-sm text-muted-foreground">{t('streamHealth.overallStatus')}:</span>
                        <OverallStatusBadge status={overallStatus} />
                    </div>
                    <div class="text-sm">
                        <span class="text-muted-foreground">{t('streamHealth.streamsUp')}:</span>{' '}
                        <span class="font-semibold text-foreground">{streams.up}/{streams.total}</span>
                    </div>
                    {streams.degraded > 0 && (
                        <div class="text-sm">
                            <span class="text-muted-foreground">{t('streamHealth.degraded')}:</span>{' '}
                            <span class="font-semibold" style="color: hsl(45 80% 35%)">{streams.degraded}</span>
                        </div>
                    )}
                    {streams.down > 0 && (
                        <div class="text-sm">
                            <span class="text-muted-foreground">{t('streamHealth.down')}:</span>{' '}
                            <span class="font-semibold" style="color: hsl(0 84% 40%)">{streams.down}</span>
                        </div>
                    )}
                    <div class="text-sm ml-auto">
                        <span class="text-muted-foreground">{t('streamHealth.storage')}:</span>{' '}
                        <span class="font-semibold text-foreground">
                            {formatBytes(storage.used_bytes)} / {formatBytes((storage.used_bytes || 0) + (storage.available_bytes || 0))}
                            {' '}({storagePercent}%)
                        </span>
                    </div>
                </div>
            </div>

            {/* Stream Health Cards Grid */}
            {streamsDetail.length === 0 ? (
                <div class="text-center text-muted-foreground py-12">
                    {t('streamHealth.noStreams')}
                </div>
            ) : (
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
                    {streamsDetail.map(stream => (
                        <StreamCard
                            key={stream.name}
                            stream={stream}
                            expanded={expandedStream === stream.name}
                            onToggle={() => setExpandedStream(
                                expandedStream === stream.name ? null : stream.name
                            )}
                        />
                    ))}
                </div>
            )}
        </div>
    );
}
