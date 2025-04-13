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
        // We'll get the client ID from the server's welcome message
        this.clientId = null;
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
        // Store callbacks waiting for client ID
        this.clientIdCallbacks = [];
        // Connection change listeners
        this.connectionChangeListeners = [];

        console.log('WebSocket client initialized, waiting for server-assigned client ID');

        // Bind methods to this instance
        this.connect = this.connect.bind(this);
        this.disconnect = this.disconnect.bind(this);
        this.reconnect = this.reconnect.bind(this);
        this.send = this.send.bind(this);
        this.subscribe = this.subscribe.bind(this);
        this.unsubscribe = this.unsubscribe.bind(this);
        this.handleMessage = this.handleMessage.bind(this);
        this.processMessageQueue = this.processMessageQueue.bind(this);
        this.addConnectionChangeListener = this.addConnectionChangeListener.bind(this);
        this.removeConnectionChangeListener = this.removeConnectionChangeListener.bind(this);
        this.notifyConnectionChangeListeners = this.notifyConnectionChangeListeners.bind(this);

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

                // Notify connection change listeners
                this.notifyConnectionChangeListeners(true);

                // Set a smaller buffer size for better compatibility with older systems
                if (this.socket.bufferedAmount !== undefined) {
                    console.log(`Initial WebSocket buffered amount: ${this.socket.bufferedAmount}`);
                }

                // Wait for welcome message with client ID before processing subscriptions
                // The server will send a welcome message with the client ID
                console.log('WebSocket connected, waiting for welcome message with client ID');

                // If we already have a client ID (from a previous connection), we can process
                // pending subscriptions and messages immediately
                if (this.clientId) {
                    console.log(`Using existing client ID: ${this.clientId}`);
                    // Resubscribe to topics
                    this.pendingSubscriptions = new Set([...this.subscriptions]);
                    this.subscriptions.clear();
                    this.processPendingSubscriptions();

                    // Process any queued messages
                    this.processMessageQueue();
                }
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

                // Notify connection change listeners
                this.notifyConnectionChangeListeners(false);

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

                // If we were connected, notify listeners that we're disconnected
                if (this.connected) {
                    this.connected = false;
                    this.notifyConnectionChangeListeners(false);
                }

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

        // Make sure connected is set to false
        if (this.connected) {
            this.connected = false;
            // Notify connection change listeners
            this.notifyConnectionChangeListeners(false);
        }

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
        // Keep the client ID if we have one, so we can reuse it on reconnect
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
                    console.log(`WebSocket client ID received from server: ${this.clientId}`);

                    // Process pending subscriptions
                    this.pendingSubscriptions = new Set([...this.subscriptions]);
                    this.subscriptions.clear();
                    this.processPendingSubscriptions();

                    // Process any queued messages
                    this.processMessageQueue();

                    // Notify any waiting operations
                    console.log(`Notifying ${this.clientIdCallbacks.length} waiting operations about client ID`);

                    // Call all callbacks waiting for client ID
                    const callbacks = [...this.clientIdCallbacks];
                    this.clientIdCallbacks = [];
                    for (const callback of callbacks) {
                        try {
                            callback(this.clientId);
                        } catch (error) {
                            console.error('Error in client ID callback:', error);
                        }
                    }
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
     * @param {Function} callback Optional callback to call when client ID is available
     * @returns {string|null} Client ID or null if not connected
     */
    getClientId(callback) {
        if (this.clientId) {
            // If we already have a client ID, return it immediately
            if (callback) {
                callback(this.clientId);
            }
            return this.clientId;
        } else if (callback) {
            // If we don't have a client ID yet, add the callback to the queue
            console.log('Client ID not available yet, adding callback to queue');
            this.clientIdCallbacks.push(callback);

            // Make sure we're connected to get a client ID
            if (!this.connected && !this.connecting) {
                console.log('Not connected, connecting now to get client ID');
                this.connect();
            }
            return null;
        }
        return null;
    }

    /**
     * Add a connection change listener
     *
     * @param {Function} listener Function to call when connection state changes
     */
    addConnectionChangeListener(listener) {
        if (typeof listener !== 'function') {
            console.error('Connection change listener must be a function');
            return;
        }

        // Add listener if it doesn't already exist
        if (!this.connectionChangeListeners.includes(listener)) {
            this.connectionChangeListeners.push(listener);
            console.log('Added connection change listener');

            // Call the listener immediately with the current connection state
            try {
                listener(this.connected);
            } catch (error) {
                console.error('Error in connection change listener:', error);
            }
        }
    }

    /**
     * Remove a connection change listener
     *
     * @param {Function} listener Function to remove
     */
    removeConnectionChangeListener(listener) {
        const index = this.connectionChangeListeners.indexOf(listener);
        if (index !== -1) {
            this.connectionChangeListeners.splice(index, 1);
            console.log('Removed connection change listener');
        }
    }

    /**
     * Notify all connection change listeners
     *
     * @param {boolean} connected Whether the connection is established
     */
    notifyConnectionChangeListeners(connected) {
        console.log(`Notifying ${this.connectionChangeListeners.length} connection change listeners: connected=${connected}`);

        for (const listener of this.connectionChangeListeners) {
            try {
                listener(connected);
            } catch (error) {
                console.error('Error in connection change listener:', error);
            }
        }
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
        this._httpFallbackTimeout = null; // Store timeout ID for HTTP fallback

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

            // Get client ID from the WebSocket client
            // This will either return the client ID immediately or call our callback when it's available
            const getClientIdAndProceed = (clientId) => {
                console.log('Using WebSocket client ID for batch delete:', clientId);

                console.log('Registering WebSocket handlers for batch delete');

    // Register one-time result handler
    const resultHandler = (payload) => {
        console.log('Batch delete result received:', payload);
        this.wsClient.off('result', this.topic, resultHandler);

        // Log the full payload details for debugging
        console.log('Batch delete result details:', JSON.stringify(payload, null, 2));

        // Clear any pending HTTP fallback timeouts
        if (this._httpFallbackTimeout) {
            console.log('Clearing HTTP fallback timeout because we received a result');
            clearTimeout(this._httpFallbackTimeout);
            this._httpFallbackTimeout = null;
        }

        resolve(payload);
    };

    // Register one-time error handler
    const errorHandler = (payload) => {
        console.error('Batch delete error received:', payload);
        this.wsClient.off('error', this.topic, errorHandler);

        // Clear any pending HTTP fallback timeouts
        if (this._httpFallbackTimeout) {
            console.log('Clearing HTTP fallback timeout because we received an error');
            clearTimeout(this._httpFallbackTimeout);
            this._httpFallbackTimeout = null;
        }

        reject(new Error(payload.error || 'Unknown error'));
    };

                // Register handlers for both result and progress
                this.wsClient.on('result', this.topic, resultHandler);
                this.wsClient.on('error', this.topic, errorHandler);

                // Make sure we're subscribed to the topic
                // Use an empty object as payload to avoid issues
                if (!this.wsClient.subscriptions.has(this.topic)) {
                    console.log(`Subscribing to ${this.topic} before sending request`);

                    // Create a subscription message with client_id
                    const subscribePayload = { client_id: clientId };
                    console.log(`Subscription payload:`, subscribePayload);

                    // Send subscription directly to ensure it works
                    if (this.wsClient.socket && this.wsClient.socket.readyState === WebSocket.OPEN) {
                        const subscribeMsg = {
                            type: 'subscribe',
                            topic: this.topic,
                            payload: subscribePayload
                        };
                        this.wsClient.socket.send(JSON.stringify(subscribeMsg));
                        console.log('Subscription sent directly via socket');
                    } else {
                        this.wsClient.send('subscribe', this.topic, subscribePayload);
                    }

                    this.wsClient.subscriptions.add(this.topic);

                    // Add a longer delay to ensure subscription is processed
                    setTimeout(() => {
                        // Send the actual delete request with client ID
                        this.sendDeleteRequest(params, clientId, resolve, reject, resultHandler, errorHandler);
                    }, 1000);
                } else {
                    console.log(`Already subscribed to ${this.topic}, sending request immediately`);
                    // Already subscribed, send request immediately
                    this.sendDeleteRequest(params, clientId, resolve, reject, resultHandler, errorHandler);
                }
            };

            // Get the client ID, which will either return it immediately or call our callback when available
            const clientId = this.wsClient.getClientId(getClientIdAndProceed);
            if (clientId) {
                // If we got the client ID immediately, proceed with the operation
                getClientIdAndProceed(clientId);
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

        // Validate client ID - it should be a pointer value from the server
        if (!clientId || !clientId.startsWith('0x')) {
            console.error(`Invalid client ID format: ${clientId}. Expected a pointer value like 0x12345678`);
            this.wsClient.off('result', this.topic, resultHandler);
            this.wsClient.off('error', this.topic, errorHandler);
            reject(new Error('Invalid client ID format. Please try again.'));
            return;
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

        // Debug the client ID format
        console.log('Client ID format check:', {
            clientId,
            startsWithHex: clientId.startsWith('0x'),
            length: clientId.length,
            isValid: clientId.startsWith('0x') && clientId.length > 2
        });

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
        } else if (params.ids && typeof window.updateBatchDeleteProgress === 'function') {
            // Also simulate initial progress for IDs-based delete
            console.log('Simulating initial progress update with IDs count:', params.ids.length);
            window.updateBatchDeleteProgress({
                current: 0,
                total: params.ids.length,
                succeeded: 0,
                failed: 0,
                status: `Starting batch delete operation for ${params.ids.length} recordings...`,
                complete: false
            });
        }

        // Send the request
        try {
            if (this.wsClient.socket && this.wsClient.socket.readyState === WebSocket.OPEN) {
                // Double check the client ID is included in the payload
                if (!message.payload.client_id || message.payload.client_id !== clientId) {
                    console.warn('Client ID missing or incorrect in payload, fixing it');
                    message.payload.client_id = clientId;
                }

                // Send directly to ensure it works
                this.wsClient.socket.send(JSON.stringify(message));
                console.log('Batch delete request sent successfully via direct socket send');

                // Set a longer timeout to check if we get a response
                // Some operations might take longer to start processing
                this._httpFallbackTimeout = setTimeout(() => {
                    // If we haven't received a response after 30 seconds, try HTTP fallback
                    console.warn('No response received after 30 seconds, trying HTTP fallback');
                    this._httpFallbackTimeout = null;
                    if (typeof batchDeleteRecordingsByHttpRequest === 'function') {
                        console.log('Falling back to HTTP for batch delete');
                        batchDeleteRecordingsByHttpRequest(params)
                            .then(resolve)
                            .catch(reject);
                    }
                }, 30000); // Increased from 10000 to 30000 ms to allow more time for server processing

                return true;
            } else {
                // Fall back to the send method
                const success = this.wsClient.send('request', this.topic, requestParams);

                if (!success) {
                    console.error('Failed to send batch delete request');
                    this.wsClient.off('result', this.topic, resultHandler);
                    this.wsClient.off('error', this.topic, errorHandler);

                    // Try HTTP fallback
                    if (typeof batchDeleteRecordingsByHttpRequest === 'function') {
                        console.log('Falling back to HTTP for batch delete');
                        batchDeleteRecordingsByHttpRequest(params)
                            .then(resolve)
                            .catch(reject);
                    } else {
                        reject(new Error('Failed to send batch delete request'));
                    }
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

        // If we received a progress update, we know the WebSocket connection is working
        // Clear any pending HTTP fallback timeouts
        if (this._httpFallbackTimeout) {
            console.log('Clearing HTTP fallback timeout because we received a progress update');
            clearTimeout(this._httpFallbackTimeout);
            this._httpFallbackTimeout = null;
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

        // If we received a result, we know the WebSocket connection is working
        // Clear any pending HTTP fallback timeouts
        if (this._httpFallbackTimeout) {
            console.log('Clearing HTTP fallback timeout because we received a result');
            clearTimeout(this._httpFallbackTimeout);
            this._httpFallbackTimeout = null;
        }

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

        // If we received an error, we know the WebSocket connection is working
        // Clear any pending HTTP fallback timeouts
        if (this._httpFallbackTimeout) {
            console.log('Clearing HTTP fallback timeout because we received an error');
            clearTimeout(this._httpFallbackTimeout);
            this._httpFallbackTimeout = null;
        }

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

        // Update the progress UI to show the error
        if (typeof window.updateBatchDeleteProgress === 'function') {
            try {
                // Create a progress update from the error data
                const progressUpdate = {
                    current: 0,
                    total: 0,
                    succeeded: 0,
                    failed: 0,
                    status: `Error: ${errorData.error || 'Unknown error'}`,
                    complete: true,
                    error: true
                };

                console.log('Updating progress UI with error:', progressUpdate);
                window.updateBatchDeleteProgress(progressUpdate);
            } catch (error) {
                console.error('Error updating progress UI from error:', error);
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
