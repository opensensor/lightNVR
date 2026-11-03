// @ts-nocheck
/**
 * Sparkline - Inline SVG sparkline component for stream health metrics
 * Zero dependencies, renders a simple polyline chart.
 */
import { useMemo } from 'preact/hooks';

export function Sparkline({ data = [], width = 120, height = 30, color = 'hsl(var(--primary))' }) {
    const points = useMemo(() => {
        if (!data || data.length < 2) return '';
        const max = Math.max(...data, 0.001); // avoid division by zero
        const min = Math.min(...data);
        const range = max - min || 1;
        const padding = 1;
        const w = width - padding * 2;
        const h = height - padding * 2;

        return data
            .map((val, i) => {
                const x = padding + (i / (data.length - 1)) * w;
                const y = padding + h - ((val - min) / range) * h;
                return `${x.toFixed(1)},${y.toFixed(1)}`;
            })
            .join(' ');
    }, [data, width, height]);

    if (!data || data.length < 2) {
        return (
            <svg width={width} height={height} class="inline-block opacity-30">
                <line x1="0" y1={height / 2} x2={width} y2={height / 2}
                      stroke="currentColor" stroke-width="1" stroke-dasharray="2,2" />
            </svg>
        );
    }

    return (
        <svg width={width} height={height} class="inline-block">
            <polyline
                points={points}
                fill="none"
                stroke={color}
                stroke-width="1.5"
                stroke-linejoin="round"
                stroke-linecap="round"
            />
        </svg>
    );
}
