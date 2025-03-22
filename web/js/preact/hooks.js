/**
 * Preact Hooks
 * This file exports hooks from the Preact library
 */

import { options } from '../preact.min.js';

// Keep track of current component
let currentComponent;
let currentHook = 0;

// Store component hooks
const componentHooks = new Map();

// Set up options hook to track current component
options.__r = (component) => {
  currentComponent = component;
  currentHook = 0;
};

/**
 * useState hook
 * @param {*} initialState - Initial state value
 * @returns {[*, Function]} State value and setter function
 */
export function useState(initialState) {
  const component = currentComponent;
  const hookId = currentHook++;
  
  // Get component's hooks or initialize
  const hooks = componentHooks.get(component) || [];
  if (!componentHooks.has(component)) {
    componentHooks.set(component, hooks);
  }
  
  // Initialize hook if needed
  if (hookId >= hooks.length) {
    const setState = (newState) => {
      if (typeof newState === 'function') {
        hooks[hookId][0] = newState(hooks[hookId][0]);
      } else {
        hooks[hookId][0] = newState;
      }
      
      // Force component update
      if (component && typeof component.setState === 'function') {
        component.setState({});
      }
    };
    
    // Initialize state
    const initialValue = typeof initialState === 'function' ? initialState() : initialState;
    hooks[hookId] = [initialValue, setState];
  }
  
  return hooks[hookId];
}

/**
 * useEffect hook
 * @param {Function} effect - Effect function
 * @param {Array} deps - Dependency array
 */
export function useEffect(effect, deps) {
  const component = currentComponent;
  const hookId = currentHook++;
  
  // Get component's hooks or initialize
  const hooks = componentHooks.get(component) || [];
  if (!componentHooks.has(component)) {
    componentHooks.set(component, hooks);
  }
  
  // Check if deps changed
  const hasNoDeps = !deps;
  const hasDepsChanged = hooks[hookId] ? !deps.every((dep, i) => dep === hooks[hookId][0][i]) : true;
  
  // Store deps
  hooks[hookId] = [deps, null];
  
  // Setup effect
  if (hasNoDeps || hasDepsChanged) {
    // Clean up previous effect
    if (hooks[hookId] && hooks[hookId][1]) {
      try {
        hooks[hookId][1]();
      } catch (e) {
        console.error('Error in useEffect cleanup:', e);
      }
    }
    
    // Run effect
    const cleanup = effect();
    hooks[hookId][1] = cleanup;
    
    // Clean up on unmount
    if (component && !component.__u) {
      component.__u = component.componentWillUnmount;
      component.componentWillUnmount = () => {
        if (hooks[hookId] && hooks[hookId][1]) {
          try {
            hooks[hookId][1]();
          } catch (e) {
            console.error('Error in useEffect cleanup:', e);
          }
        }
        if (component.__u) component.__u();
      };
    }
  }
}

/**
 * useRef hook
 * @param {*} initialValue - Initial ref value
 * @returns {Object} Ref object
 */
export function useRef(initialValue) {
  const component = currentComponent;
  const hookId = currentHook++;
  
  // Get component's hooks or initialize
  const hooks = componentHooks.get(component) || [];
  if (!componentHooks.has(component)) {
    componentHooks.set(component, hooks);
  }
  
  // Initialize hook if needed
  if (hookId >= hooks.length) {
    hooks[hookId] = { current: initialValue };
  }
  
  return hooks[hookId];
}

/**
 * useCallback hook
 * @param {Function} callback - Callback function
 * @param {Array} deps - Dependency array
 * @returns {Function} Memoized callback
 */
export function useCallback(callback, deps) {
  const component = currentComponent;
  const hookId = currentHook++;
  
  // Get component's hooks or initialize
  const hooks = componentHooks.get(component) || [];
  if (!componentHooks.has(component)) {
    componentHooks.set(component, hooks);
  }
  
  // Check if deps changed
  const hasNoDeps = !deps;
  const hasDepsChanged = hooks[hookId] ? !deps.every((dep, i) => dep === hooks[hookId][0][i]) : true;
  
  // Update callback if deps changed
  if (hasNoDeps || hasDepsChanged || !hooks[hookId]) {
    hooks[hookId] = [deps, callback];
  }
  
  return hooks[hookId][1];
}

/**
 * useMemo hook
 * @param {Function} factory - Factory function
 * @param {Array} deps - Dependency array
 * @returns {*} Memoized value
 */
export function useMemo(factory, deps) {
  const component = currentComponent;
  const hookId = currentHook++;
  
  // Get component's hooks or initialize
  const hooks = componentHooks.get(component) || [];
  if (!componentHooks.has(component)) {
    componentHooks.set(component, hooks);
  }
  
  // Check if deps changed
  const hasNoDeps = !deps;
  const hasDepsChanged = hooks[hookId] ? !deps.every((dep, i) => dep === hooks[hookId][0][i]) : true;
  
  // Update value if deps changed
  if (hasNoDeps || hasDepsChanged || !hooks[hookId]) {
    hooks[hookId] = [deps, factory()];
  }
  
  return hooks[hookId][1];
}

/**
 * useReducer hook
 * @param {Function} reducer - Reducer function
 * @param {*} initialState - Initial state
 * @param {Function} init - Optional init function
 * @returns {[*, Function]} State and dispatch function
 */
export function useReducer(reducer, initialState, init) {
  const component = currentComponent;
  const hookId = currentHook++;
  
  // Get component's hooks or initialize
  const hooks = componentHooks.get(component) || [];
  if (!componentHooks.has(component)) {
    componentHooks.set(component, hooks);
  }
  
  // Initialize hook if needed
  if (hookId >= hooks.length) {
    const initialValue = init ? init(initialState) : initialState;
    
    const dispatch = (action) => {
      const nextState = reducer(hooks[hookId][0], action);
      if (nextState !== hooks[hookId][0]) {
        hooks[hookId][0] = nextState;
        
        // Force component update
        if (component && typeof component.setState === 'function') {
          component.setState({});
        }
      }
    };
    
    hooks[hookId] = [initialValue, dispatch];
  }
  
  return hooks[hookId];
}

/**
 * useContext hook
 * @param {Object} context - Context object
 * @returns {*} Context value
 */
export function useContext(context) {
  currentHook++;
  return context.value;
}
