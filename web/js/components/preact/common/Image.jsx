import { useState, useEffect } from 'preact/hooks';

/**
 * Image component with preloading support
 * Ensures the image is fully loaded before displaying it
 * 
 * @param {Object} props
 * @param {string} props.src - Image source URL
 * @param {string} props.alt - Alt text for the image
 * @param {string} props.class - CSS classes
 * @param {Function} props.onError - Error handler
 * @param {Function} props.onLoad - Load handler
 * @param {boolean} props.eager - If true, preload immediately (default: false)
 * @param {JSX.Element} props.placeholder - Custom placeholder element
 * @param {Object} props - Other props passed to img element
 */
export default function Image({ 
  src, 
  alt = '', 
  class: className = '',
  onError,
  onLoad,
  eager = false,
  placeholder,
  ...otherProps 
}) {
  const [imageLoaded, setImageLoaded] = useState(false);
  const [imageError, setImageError] = useState(false);

  useEffect(() => {
    if (!src) {
      setImageError(true);
      return;
    }

    // Reset state when src changes
    setImageLoaded(false);
    setImageError(false);

    // Preload the image
    const img = new window.Image();
    
    img.onload = () => {
      setImageLoaded(true);
      if (onLoad) onLoad();
    };
    
    img.onerror = () => {
      setImageError(true);
      if (onError) onError();
    };
    
    img.src = src;

    // Cleanup
    return () => {
      img.onload = null;
      img.onerror = null;
    };
  }, [src, onLoad, onError]);

  // Show error placeholder
  if (imageError) {
    if (placeholder) {
      return placeholder;
    }
    return (
      <div class={`w-full h-full flex items-center justify-center text-muted-foreground ${className}`}>
        <svg class="w-12 h-12 opacity-30" fill="currentColor" viewBox="0 0 20 20">
          <path fillRule="evenodd" d="M4 3a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V5a2 2 0 00-2-2H4zm12 12H4l4-8 3 6 2-4 3 6z" clipRule="evenodd" />
        </svg>
      </div>
    );
  }

  // Show loading placeholder or image with opacity
  return (
    <>
      {!imageLoaded && placeholder && placeholder}
      <img
        src={src}
        alt={alt}
        class={`${className} ${imageLoaded ? 'opacity-100' : 'opacity-0'}`}
        style={{ transition: 'opacity 300ms' }}
        {...otherProps}
      />
    </>
  );
}

