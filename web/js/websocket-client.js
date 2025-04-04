/**
 * WebSocket Client for LightNVR
 * Provides a reusable WebSocket connection with automatic reconnection
 */

/**
 * WebSocket Client class
 */
export class WebSocketClient {
    /**
     * Generate a UUID v4
     * @returns {string} A random UUID
     */
    static generateUUID() {
        return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
            const r = Math.random() * 16 | 0;
            const v = c === 'x' ? r : (r & 0x3 | 0x8);
            return v.toString(16);
        });
    }
    
    /**
     * Create a new WebSocket client
     */
    constructor() {
        this.socket = null;
        // Generate a client ID locally instead of waiting for one from the server
        this.clientId = WebSocketClient.generateUUID();
        this.connected = false;
        this.reconnectAttempts = 0;
        this.maxReconnectAttempts = 5;
        this.reconnectDelay = 1000; // Start with 1 second delay
        this.handlers = {};
        this.subscriptions = new Set();
        this.pendingSubscriptions = new Set();
        this.messageQueue = [];
        this.connecting = false;
        // Store subscription parameters
        this.subscriptionParams = new Map();
        
        console.log(`Generated local WebSocket client ID: ${this.clientId}`);
        
        // Try to load client ID from localStorage if available
        try {
            const storedClientId = localStorage.getItem('websocket_client_id');
            if (storedClientId) {
                this.clientId = storedClientId;
                console.log(`Loaded WebSocket client ID from localStorage: ${this.clientId}`);
            } else {
                // Store the generated client ID
                localStorage.setItem('websocket_client_id', this.clientId);
                console.log(`Stored WebSocket client ID in localStorage: ${this.clientId}`);
            }
        } catch (e) {
            console.warn('Could not access localStorage for client ID:', e);
        }
        
        // Bind methods to this instance
        this.connect = this.connect.bind(this);
        this.disconnect = this.disconnect.bind(this);
        this.reconnect = this.reconnect.bind(this);
        this.send = this.send.bind(this);
        this.subscribe = this.subscribe.bind(this);
        this.unsubscribe = this.unsubscribe.bind(this);
        this.handleMessage = this.handleMessage.bind(this);
        this.processMessageQueue = this.processMessageQueue.bind(this);
        
        // Connect automatically
        this.connect();
    }
    
    /**
     * Connect to the WebSocket server
     */
    connect() {
        if (this.socket && (this.socket.readyState === WebSocket.OPEN || this.socket.readyState === WebSocket.CONNECTING)) {
            console.log('WebSocket already connected or connecting');
            return;
        }
        
        if (this.connecting) {
            console.log('WebSocket connection already in progress');
            return;
        }
        
        this.connecting = true;
        
        // Determine WebSocket URL based on current page protocol
        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        const wsUrl = `${protocol}//${window.location.host}/api/ws`;
        
        console.log(`Connecting to WebSocket server at ${wsUrl}`);
        
        // Check if WebSocket is supported
        if (typeof WebSocket === 'undefined') {
            console.error('WebSocket not supported by this browser or environment');
            this.connecting = false;
            this.fallbackToHttp();
            return;
        }
        
        try {
            // Create WebSocket with a timeout to handle stalled connections
            const connectTimeout = setTimeout(() => {
                console.error('WebSocket connection timeout');
                if (this.socket && this.socket.readyState !== WebSocket.OPEN) {
                    this.socket.close();
                    this.socket = null;
                    this.connecting = false;
                    this.fallbackToHttp();
                }
            }, 10000); // 10 second timeout
            
            this.socket = new WebSocket(wsUrl);
            
            this.socket.onopen = () => {
                clearTimeout(connectTimeout);
                console.log('WebSocket connection established');
                this.connected = true;
                this.connecting = false;
                this.reconnectAttempts = 0;
                this.reconnectDelay = 1000;
                
                // Set a smaller buffer size for better compatibility with older systems
                if (this.socket.bufferedAmount !== undefined) {
                    console.log(`Initial WebSocket buffered amount: ${this.socket.bufferedAmount}`);
                }
                
                // Resubscribe to topics
                this.pendingSubscriptions = new Set([...this.subscriptions]);
                this.subscriptions.clear();
                this.processPendingSubscriptions();
                
                // Process any queued messages
                this.processMessageQueue();
            };
            
            this.socket.onmessage = (event) => {
                try {
                    this.handleMessage(event.data);
                } catch (error) {
                    console.error('Error handling WebSocket message:', error);
                }
            };
            
            this.socket.onclose = (event) => {
                clearTimeout(connectTimeout);
                console.log(`WebSocket connection closed: ${event.code} ${event.reason}`);
                this.connected = false;
                this.connecting = false;
                
                // Check for specific close codes that indicate compatibility issues
                if (event.code === 1006) {
                    console.warn('WebSocket closed abnormally (code 1006), possible compatibility issue');
                    
                    // If we've tried multiple times and keep getting 1006, fall back to HTTP
                    if (this.reconnectAttempts >= 3) {
                        console.warn('Multiple abnormal closures, falling back to HTTP');
                        this.fallbackToHttp();
                        return;
                    }
                }
                
                // Attempt to reconnect if not a normal closure
                if (event.code !== 1000) {
                    this.reconnect();
                }
            };
            
            this.socket.onerror = (error) => {
                console.error('WebSocket error:', error);
                this.connecting = false;
                
                // On error, increment reconnect attempts
                this.reconnectAttempts++;
                
                // If we've had multiple errors, fall back to HTTP
                if (this.reconnectAttempts >= 3) {
                    console.warn('Multiple WebSocket errors, falling back to HTTP');
                    this.fallbackToHttp();
                }
            };
        } catch (error) {
            console.error('Error creating WebSocket:', error);
            this.connecting = false;
            this.reconnect();
        }
    }
    
    /**
     * Fall back to HTTP for operations
     * This sets a flag that other components can check to use HTTP instead of WebSocket
     */
    fallbackToHttp() {
        console.warn('Falling back to HTTP for operations');
        this.usingHttpFallback = true;
        
        // Dispatch an event that components can listen for
        const fallbackEvent = new CustomEvent('websocket-fallback', {
            detail: { usingHttp: true }
        });
        window.dispatchEvent(fallbackEvent);
        
        // Try to notify any waiting operations
        if (this.pendingSubscriptions.size > 0) {
            console.log(`Notifying ${this.pendingSubscriptions.size} pending subscriptions about HTTP fallback`);
        }
    }
    
    /**
     * Disconnect from the WebSocket server
     */
    disconnect() {
        if (this.socket) {
            this.socket.close(1000, 'Client disconnected');
            this.socket = null;
        }
        
        this.connected = false;
        this.connecting = false;
        // Don't reset clientId on disconnect since we're generating it locally
        // this.clientId = null;
        this.subscriptions.clear();
        this.pendingSubscriptions.clear();
        this.subscriptionParams.clear();
        this.messageQueue = [];
    }
    
    /**
     * Reconnect to the WebSocket server with exponential backoff
     */
    reconnect() {
        if (this.reconnectAttempts >= this.maxReconnectAttempts) {
            console.log('Maximum reconnect attempts reached');
            return;
        }
        
        this.reconnectAttempts++;
        const delay = Math.min(30000, this.reconnectDelay * Math.pow(1.5, this.reconnectAttempts - 1));
        
        console.log(`Reconnecting in ${delay}ms (attempt ${this.reconnectAttempts}/${this.maxReconnectAttempts})`);
        
        setTimeout(() => {
            this.connect();
        }, delay);
    }
    
    /**
     * Send a message to the WebSocket server
     * 
     * @param {string} type Message type
     * @param {string} topic Message topic
     * @param {Object} payload Message payload
     * @returns {boolean} Whether the message was sent
     */
    send(type, topic, payload) {
        if (!this.connected) {
            console.log('WebSocket not connected, queueing message');
            this.messageQueue.push({ type, topic, payload });
            this.connect();
            return false;
        }
        
        const message = {
            type,
            topic,
            payload
        };
        
        try {
            // Log the message being sent for debugging
            console.log('Sending WebSocket message:', JSON.stringify(message));
            
            this.socket.send(JSON.stringify(message));
            return true;
        } catch (error) {
            console.error('Error sending WebSocket message:', error);
            this.messageQueue.push({ type, topic, payload });
            return false;
        }
    }
    
    /**
     * Process the message queue
     */
    processMessageQueue() {
        if (!this.connected || this.messageQueue.length === 0) {
            return;
        }
        
        console.log(`Processing ${this.messageQueue.length} queued messages`);
        
        const queue = [...this.messageQueue];
        this.messageQueue = [];
        
        for (const message of queue) {
            this.send(message.type, message.topic, message.payload);
        }
    }
    
    /**
     * Process pending subscriptions
     */
    processPendingSubscriptions() {
        if (!this.connected || this.pendingSubscriptions.size === 0) {
            return;
        }
        
        console.log(`Processing ${this.pendingSubscriptions.size} pending subscriptions`);
        
        for (const topic of this.pendingSubscriptions) {
            // Get parameters for this topic if available
            const params = this.subscriptionParams.get(topic) || {};
            this.subscribe(topic, params);
        }
        
        this.pendingSubscriptions.clear();
    }
    
    /**
     * Subscribe to a topic
     * 
     * @param {string} topic Topic to subscribe to
     * @param {Object} params Additional parameters to include in the subscription
     * @returns {boolean} Whether the subscription request was sent
     */
    subscribe(topic, params = {}) {
        if (!this.connected) {
            console.log(`WebSocket not connected, queueing subscription to ${topic}`);
            this.pendingSubscriptions.add(topic);
            this.connect();
            return false;
        }
        
        if (this.subscriptions.has(topic)) {
            console.log(`Already subscribed to ${topic}, updating parameters`);
            // Continue with the subscription to update parameters
        }
        
        console.log(`Subscribing to ${topic} with params:`, params);
        
        // Store parameters for this topic
        this.subscriptionParams.set(topic, params);
        
        // Include client_id in the payload to ensure server knows who's subscribing
        const payload = {
            client_id: this.clientId,
            ...params
        };
        
        const success = this.send('subscribe', topic, payload);
        if (success) {
            this.subscriptions.add(topic);
        } else {
            this.pendingSubscriptions.add(topic);
        }
        
        return success;
    }
    
    /**
     * Unsubscribe from a topic
     * 
     * @param {string} topic Topic to unsubscribe from
     * @returns {boolean} Whether the unsubscription request was sent
     */
    unsubscribe(topic) {
        if (!this.connected) {
            console.log(`WebSocket not connected, cannot unsubscribe from ${topic}`);
            this.pendingSubscriptions.delete(topic);
            return false;
        }
        
        if (!this.subscriptions.has(topic)) {
            console.log(`Not subscribed to ${topic}`);
            return true;
        }
        
        console.log(`Unsubscribing from ${topic}`);
        
        const success = this.send('unsubscribe', topic, {});
        if (success) {
            this.subscriptions.delete(topic);
            // Remove stored parameters for this topic
            this.subscriptionParams.delete(topic);
        }
        
        return success;
    }
    
    /**
     * Handle an incoming WebSocket message
     * 
     * @param {string} data Message data
     */
    handleMessage(data) {
        console.log('Raw WebSocket message received:', data);
        
        try {
            const message = JSON.parse(data);
            
            if (!message.type || !message.topic) {
                console.error('Invalid WebSocket message format:', message);
                return;
            }
            
            console.log(`Received WebSocket message: ${message.type} ${message.topic}`, message);
            
            // Debug log for progress and result messages
            if (message.type === 'progress' || message.type === 'result') {
                console.log(`Received ${message.type} message for topic ${message.topic}:`, message.payload);
            }
            
            // Always try to parse payload if it's a string (could be JSON)
            if (typeof message.payload === 'string') {
                try {
                    const parsedPayload = JSON.parse(message.payload);
                    console.log(`Parsed payload for ${message.type}:${message.topic}:`, parsedPayload);
                    // Replace the string payload with the parsed object
                    message.payload = parsedPayload;
                } catch (e) {
                    // Not JSON, keep as string
                    console.log(`Payload for ${message.type}:${message.topic} is not JSON, keeping as string`);
                }
            }
            
            // Handle welcome message
            if (message.type === 'welcome' && message.topic === 'system') {
                console.log('Received welcome message:', message);
                
                // The payload might be a string that needs to be parsed
                let payload = message.payload;
                console.log('Welcome payload type:', typeof payload);
                
                // If payload is a string, try to parse it as JSON
                if (typeof payload === 'string') {
                    try {
                        payload = JSON.parse(payload);
                        console.log('Parsed welcome payload:', payload);
                    } catch (e) {
                        console.error('Error parsing welcome payload as JSON:', e);
                        console.log('Raw payload string:', payload);
                    }
                }
                
                if (!payload || !payload.client_id) {
                    console.error('Welcome message missing client_id:', payload);
                } else {
                    // Extract client ID from payload
                    this.clientId = payload.client_id;
                    console.log(`WebSocket client ID received: ${this.clientId}`);
                    
                    // Process pending subscriptions
                    this.processPendingSubscriptions();
                    
                    // Notify any waiting operations
                    console.log('Notifying any waiting operations about client ID');
                }
                return;
            }
            
            // Handle other messages
            const key = `${message.type}:${message.topic}`;
            const handlers = this.handlers[key] || [];
            
            for (const handler of handlers) {
                try {
                    handler(message.payload);
                } catch (error) {
                    console.error(`Error in WebSocket message handler for ${key}:`, error);
                }
            }
        } catch (error) {
            console.error('Error parsing WebSocket message:', error);
        }
    }
    
    /**
     * Register a message handler
     * 
     * @param {string} type Message type
     * @param {string} topic Message topic
     * @param {Function} handler Message handler
     */
    on(type, topic, handler) {
        const key = `${type}:${topic}`;
        
        if (!this.handlers[key]) {
            this.handlers[key] = [];
        }
        
        this.handlers[key].push(handler);
        
        // Subscribe to topic if not already subscribed
        if (type !== 'welcome' && type !== 'ack' && type !== 'error') {
            this.subscribe(topic);
        }
    }
    
    /**
     * Unregister a message handler
     * 
     * @param {string} type Message type
     * @param {string} topic Message topic
     * @param {Function} handler Message handler
     */
    off(type, topic, handler) {
        const key = `${type}:${topic}`;
        
        if (!this.handlers[key]) {
            return;
        }
        
        if (handler) {
            this.handlers[key] = this.handlers[key].filter(h => h !== handler);
        } else {
            this.handlers[key] = [];
        }
        
        // Unsubscribe from topic if no more handlers
        if (this.handlers[key].length === 0 && type !== 'welcome' && type !== 'ack' && type !== 'error') {
            this.unsubscribe(topic);
        }
    }
    
    /**
     * Get the client ID
     * 
     * @returns {string|null} Client ID or null if not connected
     */
    getClientId() {
        return this.clientId;
    }
    
    /**
     * Check if connected to the WebSocket server
     * 
     * @returns {boolean} Whether connected to the WebSocket server
     */
    isConnected() {
        return this.connected;
    }
}

/**
 * BatchDeleteRecordingsClient class
 * Handles batch delete recordings operations via WebSocket
 */
export class BatchDeleteRecordingsClient {
    /**
     * Create a new BatchDeleteRecordingsClient
     * 
     * @param {WebSocketClient} wsClient WebSocket client
     */
    constructor(wsClient) {
        this.wsClient = wsClient;
        this.topic = 'recordings/batch-delete';
        this.progressHandlers = [];
        this.resultHandlers = [];
        this.errorHandlers = [];
        
        // Subscribe to topic
        this.wsClient.subscribe(this.topic);
        
        // Register handlers
        this.wsClient.on('progress', this.topic, (payload) => {
            this.handleProgress(payload);
        });
        
        this.wsClient.on('result', this.topic, (payload) => {
            this.handleResult(payload);
        });
        
        this.wsClient.on('error', this.topic, (payload) => {
            this.handleError(payload);
        });
    }
    
    /**
     * Delete recordings with progress updates
     * 
     * @param {Object} params Delete parameters (ids or filter)
     * @returns {Promise<Object>} Promise that resolves when the operation is complete
     */
    deleteWithProgress(params) {
        return new Promise((resolve, reject) => {
            console.log('Starting batch delete operation with params:', params);
            
            // Check if WebSocket is connected
            if (!this.wsClient.isConnected()) {
                console.error('WebSocket not connected, attempting to connect');
                this.wsClient.connect();
                
                // Wait for connection and retry
                setTimeout(() => {
                    if (this.wsClient.isConnected()) {
                        console.log('WebSocket connected, retrying batch delete');
                        this.deleteWithProgress(params)
                            .then(resolve)
                            .catch(reject);
                    } else {
                        console.error('WebSocket still not connected, falling back to HTTP');
                        // Fallback to HTTP if available
                        if (typeof batchDeleteRecordingsByHttpRequest === 'function') {
                            batchDeleteRecordingsByHttpRequest(params)
                                .then(resolve)
                                .catch(reject);
                        } else {
                            reject(new Error('WebSocket not connected and HTTP fallback not available'));
                        }
                    }
                }, 1000);
                return;
            }
            
            // Get client ID (should always be available now since we generate it locally)
            const clientId = this.wsClient.getClientId();
            console.log('Using WebSocket client ID:', clientId);
            
            // No need to check if client ID is available or wait for welcome message
            // since we're generating the client ID locally
            
            console.log('Registering WebSocket handlers for batch delete');
            
            // Register one-time result handler
            const resultHandler = (payload) => {
                console.log('Batch delete result received:', payload);
                this.wsClient.off('result', this.topic, resultHandler);
                
                // Log the full payload details for debugging
                console.log('Batch delete result details:', JSON.stringify(payload, null, 2));
                
                resolve(payload);
            };
            
            // Register one-time error handler
            const errorHandler = (payload) => {
                console.error('Batch delete error received:', payload);
                this.wsClient.off('error', this.topic, errorHandler);
                reject(new Error(payload.error || 'Unknown error'));
            };
            
            // Register handlers for both result and progress
            this.wsClient.on('result', this.topic, resultHandler);
            this.wsClient.on('error', this.topic, errorHandler);
            
            // Make sure we're subscribed to the topic
            // Use an empty object as payload to avoid issues
            if (!this.wsClient.subscriptions.has(this.topic)) {
                console.log(`Subscribing to ${this.topic} before sending request`);
                this.wsClient.send('subscribe', this.topic, {});
                this.wsClient.subscriptions.add(this.topic);
                
                // Add a small delay to ensure subscription is processed
                setTimeout(() => {
                    // Send the actual delete request with client ID
                    this.sendDeleteRequest(params, clientId, resolve, reject, resultHandler, errorHandler);
                }, 500);
            } else {
                // Already subscribed, send request immediately
                this.sendDeleteRequest(params, clientId, resolve, reject, resultHandler, errorHandler);
            }
        });
    }
    
    /**
     * Send the actual delete request
     * 
     * @param {Object} params Delete parameters
     * @param {string} clientId Client ID
     * @param {Function} resolve Promise resolve function
     * @param {Function} reject Promise reject function
     * @param {Function} resultHandler Result handler to unregister on failure
     * @param {Function} errorHandler Error handler to unregister on failure
     */
    sendDeleteRequest(params, clientId, resolve, reject, resultHandler, errorHandler) {
        console.log('Sending batch delete request');
        
        // Validate params
        if (!params.ids && !params.filter) {
            console.error('Missing ids or filter in batch delete params');
            this.wsClient.off('result', this.topic, resultHandler);
            this.wsClient.off('error', this.topic, errorHandler);
            reject(new Error('Missing ids or filter in batch delete params'));
            return;
        }
        
        // Log the IDs if present for debugging
        if (params.ids) {
            console.log(`Deleting ${params.ids.length} recordings with IDs:`, params.ids);
        } else if (params.filter) {
            console.log('Deleting recordings with filter:', params.filter);
        }
        
        // Include the client ID in the request payload
        const requestParams = {
            ...params,
            client_id: clientId // Add client ID to the request
        };
        
        // Log the full request details for debugging
        console.log('Full request params:', JSON.stringify(requestParams, null, 2));
        
        // Create the complete message
        const message = {
            type: 'request',
            topic: this.topic,
            payload: requestParams
        };
        
        // Log the complete message
        console.log('Complete WebSocket message:', JSON.stringify(message, null, 2));
        
        // If we have a total count from the filter, simulate an initial progress update
        // This helps the UI show progress even if the server doesn't send updates
        if (params.filter && params.totalCount && typeof window.updateBatchDeleteProgress === 'function') {
            console.log('Simulating initial progress update with total count:', params.totalCount);
            window.updateBatchDeleteProgress({
                current: 0,
                total: params.totalCount,
                succeeded: 0,
                failed: 0,
                status: `Starting batch delete operation for ${params.totalCount} recordings...`,
                complete: false
            });
        }
        
        // Send the request
        try {
            if (this.wsClient.socket && this.wsClient.socket.readyState === WebSocket.OPEN) {
                // Send directly to ensure it works
                this.wsClient.socket.send(JSON.stringify(message));
                console.log('Batch delete request sent successfully via direct socket send');
                return true;
            } else {
                // Fall back to the send method
                const success = this.wsClient.send('request', this.topic, requestParams);
                
                if (!success) {
                    console.error('Failed to send batch delete request');
                    this.wsClient.off('result', this.topic, resultHandler);
                    this.wsClient.off('error', this.topic, errorHandler);
                    reject(new Error('Failed to send batch delete request'));
                } else {
                    console.log('Batch delete request sent successfully via send method');
                }
                
                return success;
            }
        } catch (error) {
            console.error('Error sending batch delete request:', error);
            this.wsClient.off('result', this.topic, resultHandler);
            this.wsClient.off('error', this.topic, errorHandler);
            reject(error);
            return false;
        }
    }
    
    /**
     * Handle progress update
     * 
     * @param {Object} payload Progress payload
     */
    handleProgress(payload) {
        console.log('Batch delete progress update received:', payload);
        
        // Ensure payload is an object
        let progressData = payload;
        
        // If payload is still a string (wasn't parsed in handleMessage), try to parse it here
        if (typeof payload === 'string') {
            try {
                progressData = JSON.parse(payload);
                console.log('Parsed progress payload:', progressData);
            } catch (e) {
                console.error('Error parsing progress payload:', e);
                // Keep original payload if parsing fails
                progressData = { error: 'Failed to parse progress data' };
            }
        }
        
        // Directly update the progress UI with the progress data
        // This ensures the progress bar is updated even if the registered handlers don't work
        if (typeof window.updateBatchDeleteProgress === 'function' && progressData) {
            try {
                console.log('Directly updating progress UI with:', progressData);
                window.updateBatchDeleteProgress(progressData);
            } catch (error) {
                console.error('Error directly updating progress UI:', error);
            }
        }
        
        // Call all registered progress handlers with the progress data
        for (const handler of this.progressHandlers) {
            try {
                handler(progressData);
            } catch (error) {
                console.error('Error in progress handler:', error);
            }
        }
    }
    
    /**
     * Handle result
     * 
     * @param {Object} payload Result payload
     */
    handleResult(payload) {
        console.log('Batch delete result received:', payload);
        
        // Ensure payload is an object
        let resultData = payload;
        
        // If payload is still a string (wasn't parsed in handleMessage), try to parse it here
        if (typeof payload === 'string') {
            try {
                resultData = JSON.parse(payload);
                console.log('Parsed result payload:', resultData);
            } catch (e) {
                console.error('Error parsing result payload:', e);
                // Keep original payload if parsing fails
                resultData = { error: 'Failed to parse result data' };
            }
        }
        
        // Also update the progress UI with the final result data
        // This ensures the progress bar shows 100% even if no progress updates were received
        if (typeof window.updateBatchDeleteProgress === 'function' && resultData) {
            try {
                // Create a progress update from the result data
                const progressUpdate = {
                    current: resultData.total || 0,
                    total: resultData.total || 0,
                    succeeded: resultData.succeeded || 0,
                    failed: resultData.failed || 0,
                    status: 'Batch delete operation complete',
                    complete: true
                };
                
                console.log('Updating progress UI with final result:', progressUpdate);
                window.updateBatchDeleteProgress(progressUpdate);
            } catch (error) {
                console.error('Error updating progress UI from result:', error);
            }
        }
        
        // Call all registered result handlers with the result data
        for (const handler of this.resultHandlers) {
            try {
                handler(resultData);
            } catch (error) {
                console.error('Error in result handler:', error);
            }
        }
    }
    
    /**
     * Handle error
     * 
     * @param {Object} payload Error payload
     */
    handleError(payload) {
        console.log('Batch delete error received:', payload);
        
        // Ensure payload is an object
        let errorData = payload;
        
        // If payload is still a string (wasn't parsed in handleMessage), try to parse it here
        if (typeof payload === 'string') {
            try {
                errorData = JSON.parse(payload);
                console.log('Parsed error payload:', errorData);
            } catch (e) {
                console.error('Error parsing error payload:', e);
                // Keep original payload if parsing fails
                errorData = { error: typeof payload === 'string' ? payload : 'Unknown error' };
            }
        }
        
        // Call all registered error handlers with the error data
        for (const handler of this.errorHandlers) {
            try {
                handler(errorData);
            } catch (error) {
                console.error('Error in error handler:', error);
            }
        }
    }
    
    /**
     * Register progress handler
     * 
     * @param {Function} handler Progress handler
     * @returns {Function} Function to unregister the handler
     */
    onProgress(handler) {
        this.progressHandlers.push(handler);
        
        return () => {
            this.progressHandlers = this.progressHandlers.filter(h => h !== handler);
        };
    }
    
    /**
     * Register result handler
     * 
     * @param {Function} handler Result handler
     * @returns {Function} Function to unregister the handler
     */
    onResult(handler) {
        this.resultHandlers.push(handler);
        
        return () => {
            this.resultHandlers = this.resultHandlers.filter(h => h !== handler);
        };
    }
    
    /**
     * Register error handler
     * 
     * @param {Function} handler Error handler
     * @returns {Function} Function to unregister the handler
     */
    onError(handler) {
        this.errorHandlers.push(handler);
        
        return () => {
            this.errorHandlers = this.errorHandlers.filter(h => h !== handler);
        };
    }
}
