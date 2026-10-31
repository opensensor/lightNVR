import { duplicateImplementations } from './source-identity.js';

export default {
  meta: {
    type: 'problem',
    docs: { description: 'Reject multiple implementations of one extension-independent module identity' },
    schema: [],
    messages: { duplicate: 'Duplicate module implementations: {{files}}. Keep one implementation; a JS bridge must use @ts-check and only re-export its typed sibling.' },
  },
  create(context) {
    return {
      Program(node) {
        const duplicates = duplicateImplementations(context.filename);
        if (duplicates.length) context.report({ node, messageId: 'duplicate', data: { files: duplicates.join(', ') } });
      },
    };
  },
};
