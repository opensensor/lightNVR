#!/usr/bin/env python3
"""Validate local contract metadata and emit deterministic C bytes; stdlib only."""
import argparse
import hashlib
import json
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[2]
METHODS = {'get', 'put', 'post', 'delete', 'options', 'head', 'patch', 'trace'}
ACTIONS = set(re.findall(r'AUTHZ_\w+, "([a-z_.]+)"', (ROOT / 'src/core/authorization.c').read_text()))


def predicate(value):
    if not isinstance(value, dict) or value.get('version') != 1 or isinstance(value.get('version'), bool):
        raise ValueError('Authorization predicate requires integer version 1')
    kind = value.get('kind')
    expected = {'version', 'kind'} if kind in ('authenticated', 'public', 'non_scoped_identity', 'identity_management') else {'version', 'kind', 'requirements'}
    if set(value) != expected or kind not in ('authenticated', 'public', 'non_scoped_identity', 'identity_management', 'camera', 'global', 'non_scoped_global', 'non_scoped_camera'):
        raise ValueError('Unsupported authorization predicate kind or keys')
    if kind in ('camera', 'global', 'non_scoped_global', 'non_scoped_camera'):
        requirements = value['requirements']
        if not isinstance(requirements, list) or not requirements:
            raise ValueError('Authorization requirements must be nonempty')
        for req in requirements:
            if not isinstance(req, dict) or set(req) != {'action', 'resource'}:
                raise ValueError('Unsupported authorization requirement keys')
            if req['action'] not in ACTIONS or req['resource'] not in ('raw', 'runtime', 'live'):
                raise ValueError('Unsupported authorization action or resource')
            if kind in ('global', 'non_scoped_global') and req['resource'] != 'raw':
                raise ValueError('Global authorization uses raw resource placeholder')


def validate(doc, projected=False):
    if not isinstance(doc, dict) or doc.get('openapi') != '3.1.1':
        raise ValueError('Contract must use OpenAPI 3.1.1')
    if not isinstance(doc.get('paths'), dict):
        raise ValueError('Contract paths must be an object')
    def resolve_pointer(reference):
        target = doc
        for token in reference[2:].split('/'):
            if re.search(r'~(?![01])', token):
                raise ValueError('Invalid JSON Pointer escape')
            token = token.replace('~1', '/').replace('~0', '~')
            if isinstance(target, list) and re.fullmatch(r'0|[1-9][0-9]*', token):
                index = int(token)
                if index >= len(target):
                    raise ValueError(f'Unresolved local reference: {reference}')
                target = target[index]
            elif isinstance(target, dict) and token in target:
                target = target[token]
            else:
                raise ValueError(f'Unresolved local reference: {reference}')
        return target
    named_maps = {'properties', 'patternProperties', '$defs', 'definitions', 'dependentSchemas',
                  'schemas', 'securitySchemes', 'requestBodies', 'parameters', 'responses',
                  'links', 'callbacks', 'pathItems', 'examples', 'encoding', 'headers', 'paths'}
    def walk(value, named=False):
        if isinstance(value, dict):
            for key, child in value.items():
                if named:
                    walk(child)
                    continue
                if key in {'default', 'enum', 'const', 'example', 'value', 'security'}:
                    continue
                if key == 'examples' and isinstance(child, list):
                    continue
                if key in ('$dynamicRef', '$dynamicAnchor', '$recursiveRef', '$recursiveAnchor', '$anchor', '$id'):
                    raise ValueError(f'Unsupported schema feature: {key}')
                if key == '$ref':
                    if not isinstance(child, str) or not child.startswith('#/'):
                        raise ValueError('Only local JSON Pointer references are supported')
                    resolve_pointer(child)
                if key == 'operationRef':
                    if not isinstance(child, str) or not child.startswith('#/'):
                        raise ValueError('Only local operationRef pointers are supported')
                    resolve_pointer(child)
                if key == 'discriminator' and isinstance(child, dict):
                    for reference in child.get('mapping', {}).values():
                        if isinstance(reference, str) and reference.startswith('#/'):
                            resolve_pointer(reference)
                        elif reference not in doc.get('components', {}).get('schemas', {}):
                            raise ValueError('Unsupported discriminator mapping reference')
                if key in ('x-lightnvr-authorization', 'x-lightnvr-visibility'):
                    predicate(child)
                walk(child, key in named_maps)
        elif isinstance(value, list):
            for child in value:
                walk(child)
    walk(doc)
    ids = set()
    for item in doc['paths'].values():
        if not isinstance(item, dict):
            raise ValueError('Path item must be an object')
        for method, operation in item.items():
            if method not in METHODS:
                continue
            if not isinstance(operation, dict) or not isinstance(operation.get('operationId'), str):
                raise ValueError('Every operation needs a stable operationId')
            if operation['operationId'] in ids:
                raise ValueError('Duplicate operationId')
            ids.add(operation['operationId'])
            if not projected and 'x-lightnvr-authorization' not in operation:
                raise ValueError('Every canonical operation needs authorization metadata')
    schemas = doc.get('components', {}).get('schemas', {})
    selected = doc.get('x-lightnvr-validators', list(schemas))
    if not isinstance(selected, list) or len(set(selected)) != len(selected):
        raise ValueError('x-lightnvr-validators must be a unique schema name array')
    for name in selected:
        if not isinstance(name, str) or not re.fullmatch(r'[A-Za-z][A-Za-z0-9_]*', name) or name not in schemas:
            raise ValueError('Validator names must identify named component schemas')
    return doc


def generate(doc):
    payload = json.dumps(doc, sort_keys=True, separators=(',', ':'), ensure_ascii=False).encode() + b'\n'
    revision = hashlib.sha256(payload).hexdigest()
    rows = [', '.join(f'0x{b:02x}' for b in payload[i:i+16]) for i in range(0, len(payload), 16)]
    body = ',\n'.join('    ' + row for row in rows)
    return ('/* Generated by scripts/contracts/generate_native.py; do not edit. */\n'
            '#ifndef LIGHTNVR_OPENAPI_DOCUMENT_GENERATED_H\n#define LIGHTNVR_OPENAPI_DOCUMENT_GENERATED_H\n'
            '#include <stddef.h>\n'
            'static const unsigned char lightnvr_openapi_document[] = {\n' + body + ',\n    0x00\n};\n'
            'static const size_t lightnvr_openapi_document_length = sizeof(lightnvr_openapi_document) - 1;\n'
            f'static const char lightnvr_openapi_document_revision[] = "{revision}";\n'
            '#endif\n')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', default=str(ROOT / 'docs/api/openapi.json'))
    parser.add_argument('--output', default=str(ROOT / 'include/web/openapi_document_generated.h'))
    parser.add_argument('--validate-only', action='store_true')
    parser.add_argument('--projected', action='store_true')
    args = parser.parse_args()
    doc = validate(json.loads(pathlib.Path(args.source).read_text()), args.projected)
    if not args.validate_only:
        target = pathlib.Path(args.output)
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(generate(doc))

if __name__ == '__main__':
    main()
