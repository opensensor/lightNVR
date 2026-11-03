import * as Sentry from '@sentry/browser';

Sentry.init({
  dsn: import.meta.env.VITE_SENTRY_DSN,
  enabled: import.meta.env.PROD && Boolean(import.meta.env.VITE_SENTRY_DSN),
  environment: 'production',
  release: import.meta.env.VITE_SENTRY_RELEASE,
  integrations: [Sentry.browserTracingIntegration()],
  tracesSampleRate: 0.1,
  dataCollection: {
    userInfo: false, cookies: false, httpHeaders: { request: false, response: false },
    httpBodies: [], urlQueryParams: false, genAI: { inputs: false, outputs: false },
    databaseQueryData: false, graphQL: { document: false, variables: false }, stackFrameVariables: false,
  },
  beforeSend(event) {
    delete event.user;
    if (event.request) {
      delete event.request.data; delete event.request.cookies; delete event.request.headers; delete event.request.query_string;
      if (event.request.url) event.request.url = event.request.url.split(/[?#]/)[0];
    }
    return event;
  },
  beforeBreadcrumb(breadcrumb) { return breadcrumb.category === 'console' ? null : breadcrumb; },
});
