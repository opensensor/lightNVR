#!/usr/bin/env python3
"""
Enhanced SOD Library Splitter

This script precisely splits the monolithic sod.c file into component files
with improved handling for:
- Proper header dependencies and include order
- Complete forward declarations 
- Corrected preprocessor directives
- Cross-module type dependencies
- Comment preservation
- Proper API export macros

Usage:
    python enhanced_sod_splitter.py --input /path/to/sod.c --output-dir /path/to/output/
"""

import os
import re
import sys
import argparse
import shutil
from collections import defaultdict, namedtuple, Counter

# Define structure to track code elements
Element = namedtuple('Element', ['name', 'type', 'content', 'start', 'end', 'deps'])

# Define major component groups with their dependencies
COMPONENT_GROUPS = {
    # Core components
    'common': [],  # No dependencies
    'data_structures': ['common'],
    'nn_types': ['common', 'data_structures'],
    'activation': ['common', 'nn_types'],
    
    # Neural network components
    'cpu_utils': ['common', 'nn_types', 'data_structures'],
    'nn_utils': ['common', 'nn_types', 'data_structures', 'activation', 'cpu_utils'],
    'cost_layer': ['common', 'nn_types', 'data_structures', 'activation'],
    'softmax_impl': ['common', 'nn_types', 'data_structures', 'activation', 'cpu_utils'],
    'batchnorm_impl': ['common', 'nn_types', 'data_structures', 'activation', 'cpu_utils'],
    'connected_impl': ['common', 'nn_types', 'data_structures', 'activation', 'cpu_utils'],
    'convolutional': ['common', 'nn_types', 'data_structures', 'activation', 'cpu_utils'],
    'dropout': ['common', 'nn_types', 'data_structures', 'activation', 'cpu_utils'],
    'normalization': ['common', 'nn_types', 'data_structures', 'activation', 'cpu_utils'],
    'local_layer': ['common', 'nn_types', 'data_structures', 'activation', 'cpu_utils'],
    'route_layer': ['common', 'nn_types', 'data_structures', 'activation', 'cpu_utils'],
    
    # Image processing and detection
    'box_utils': ['common', 'data_structures', 'nn_types'],
    'cnn': ['common', 'nn_types', 'data_structures', 'box_utils', 'dropout'],
    'img_utils': ['common', 'data_structures', 'box_utils'],
    'detection': ['common', 'nn_types', 'data_structures', 'box_utils', 'cnn', 'img_utils', 'dropout'],
    
    # Miscellaneous utilities
    'vfs': ['common', 'data_structures'],
    'cfg_parser': ['common', 'data_structures', 'nn_types'],
    'rnn': ['common', 'nn_types', 'data_structures', 'activation', 'cpu_utils'],
}

# Define common types that need forward declarations in sod_common.h
COMMON_TYPES = [
    'network', 'layer', 'tree', 'box', 'sod_cnn', 'sod_img', 'sod_box',
    'sod_pts', 'SyBlob', 'SySet', 'SyString', 'sod_vfs',
    'sod_label_coord', 'sod_config_layer', 'IplImage', 'CvCapture',
    # Additional types found to be causing issues
    'local_layer', 'cost_layer', 'avgpool_layer', 'connected_layer',
    'convolutional_layer', 'detection_layer', 'dropout_layer',
    'maxpool_layer', 'route_layer', 'softmax_layer', 'crop_layer',
    'network_state', 'size_params', 'HANDLE', 'WIN32_FIND_DATAW'
]

# Define enums that need to be in common header
COMMON_ENUMS = [
    'SOD_CNN_LAYER_TYPE', 'ACTIVATION', 'COST_TYPE', 'learning_rate_policy',
    'SOD_REALNET_NET_TYPE', 'SOD_TR_SAMPLE_TYPE'
]

# Required constants for all files
REQUIRED_CONSTANTS = [
    'SOD_OK', 'SOD_UNSUPPORTED', 'SOD_OUTOFMEM', 'SOD_ABORT',
    'SOD_IOERR', 'SOD_LIMIT', 'SOD_APIEXPORT',
    'MIN', 'MAX', 'TWO_PI'
]

# Standard headers to include in each component
STANDARD_HEADERS = [
    "<stdlib.h>",
    "<stdint.h>", 
    "<stddef.h>",
    "<string.h>",
    "<math.h>",
    "<float.h>",
    "<stdio.h>"
]

# Define API export macro for proper function exports
SOD_API_EXPORT_MACRO = """
/* Define SOD API export macro if not already defined */
#ifndef SOD_APIEXPORT
#ifdef _WIN32
  #ifdef SOD_STATIC
    #define SOD_APIEXPORT 
  #else
    #ifdef SOD_BUILD
      #define SOD_APIEXPORT __declspec(dllexport)
    #else
      #define SOD_APIEXPORT __declspec(dllimport)
    #endif
  #endif
#else
  #define SOD_APIEXPORT
#endif
#endif /* SOD_APIEXPORT */
"""

# Required type definitions
REQUIRED_TYPEDEFS = """
/* Basic SOD data types */
typedef int sod_status;

/* Function pointer types */
typedef void (*ProcRnnCallback)(void *, int, float);
typedef void (*ProcLogCallback)(void *, int, const char *);
typedef int (*ProcLayerLoad)(void *, const char *);
typedef int (*ProcLayerExec)(void *, int);
typedef void (*ProcLayerRelease)(void *);
"""

class EnhancedSodSplitter:
    def __init__(self, input_file, output_dir):
        self.input_file = input_file
        
        # Define output directories
        self.output_dir = output_dir
        self.src_dir = os.path.join(output_dir, 'src', 'sod')
        self.include_dir = os.path.join(output_dir, 'include', 'sod')
        
        # Create necessary directory structure
        os.makedirs(self.src_dir, exist_ok=True)
        os.makedirs(self.include_dir, exist_ok=True)
        
        # Load the entire file content
        with open(input_file, 'r', encoding='utf-8', errors='ignore') as f:
            self.content = f.read()
            
        # Track all elements
        self.functions = []
        self.structs = []
        self.enums = []
        self.globals = []
        self.typedefs = []
        self.macros = []
        self.comments = []
        self.conditionals = []
        self.includes = []
        
        # Symbol to component mapping
        self.symbol_map = {}
        
        # Track module dependencies
        self.module_deps = defaultdict(set)
        
        # Output files with their content
        self.output_files = defaultdict(list)
        
        # Regular expressions
        # Improved function regex with atomic grouping to prevent catastrophic backtracking
        self.function_regex = re.compile(
            r'(?:static\s+)?(?:SOD_APIEXPORT\s+)?'
            r'(?:[a-zA-Z_][a-zA-Z0-9_*\s]+?\s+)'  # Return type - non-greedy
            r'([a-zA-Z_][a-zA-Z0-9_]*)'           # Function name
            r'\s*\([^{;]*?\)\s*{'                 # Parameters and opening brace - non-greedy
            , re.MULTILINE
        )
        self.struct_regex = re.compile(r'typedef\s+struct\s+(?:[a-zA-Z_][a-zA-Z0-9_]*\s+)?{[^}]*}(?:\s*)([a-zA-Z_][a-zA-Z0-9_]*);|struct\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*{[^}]*};', re.MULTILINE)
        self.global_regex = re.compile(r'(?:static|const|extern)?\s*(?:[a-zA-Z_][a-zA-Z0-9_]*\s+)+([a-zA-Z_][a-zA-Z0-9_]*)\s*(?:\[\s*[^\]]*\s*\])?\s*=', re.MULTILINE)
        self.typedef_regex = re.compile(r'typedef\s+(?!struct|enum)([a-zA-Z_][a-zA-Z0-9_]*(?:\s*\*)?)\s+([a-zA-Z_][a-zA-Z0-9_]*);', re.MULTILINE)
        self.enum_regex = re.compile(r'typedef\s+enum\s*(?:[a-zA-Z_][a-zA-Z0-9_]*\s*)?{[^}]*}(?:\s*)([a-zA-Z_][a-zA-Z0-9_]*);', re.MULTILINE)
        self.enum_block_regex = re.compile(r'(typedef\s+enum\s*(?:[a-zA-Z_][a-zA-Z0-9_]*\s*)?{[^}]*}(?:\s*)[a-zA-Z_][a-zA-Z0-9_]*;)', re.DOTALL)
        self.conditional_regex = re.compile(r'(#if\s+.*?|#ifdef\s+.*?|#ifndef\s+.*?)(?:#else.*?)?(?:#endif.*?)', re.DOTALL)
        self.macro_regex = re.compile(r'#define\s+([a-zA-Z_][a-zA-Z0-9_]*)', re.MULTILINE)
        self.comment_regex = re.compile(r'/\*.*?\*/|//.*?$', re.DOTALL | re.MULTILINE)
        self.include_regex = re.compile(r'#include\s+[<"]([^">]+)[">]', re.MULTILINE)

    def extract_symbols(self):
        """Extract all symbols from the source file"""
        print("Extracting symbols from source file...")
        
        # Extract comments first to better identify symbol ranges
        self.extract_comments()
        
        # Extract all code elements
        self.extract_includes()
        self.extract_enums()
        self.extract_conditionals()
        self.extract_functions()
        self.extract_structs()
        self.extract_globals()
        self.extract_typedefs()
        self.extract_macros()
        
        # Print statistics
        total_symbols = (len(self.functions) + len(self.structs) + len(self.enums) + 
                         len(self.globals) + len(self.typedefs) + len(self.macros))
        
        print(f"\nExtraction complete. Found {total_symbols} symbols:")
        print(f"  Functions: {len(self.functions)}")
        print(f"  Structs: {len(self.structs)}")
        print(f"  Enums: {len(self.enums)}")
        print(f"  Globals: {len(self.globals)}")
        print(f"  Typedefs: {len(self.typedefs)}")
        print(f"  Macros: {len(self.macros)}")
        print(f"  Comments: {len(self.comments)}")
        print(f"  Includes: {len(self.includes)}")
        print(f"  Conditionals: {len(self.conditionals)}")
        
    def extract_comments(self):
        """Extract all comments from the source file"""
        for match in self.comment_regex.finditer(self.content):
            start = match.start()
            end = match.end()
            content = match.group(0)
            self.comments.append(Element("comment", "comment", content, start, end, set()))
            
    def extract_includes(self):
        """Extract include directives from the source file"""
        for match in self.include_regex.finditer(self.content):
            start = match.start()
            end = match.end()
            content = match.group(0)
            include_file = match.group(1)
            self.includes.append(Element(include_file, "include", content, start, end, set()))
            
    def extract_enums(self):
        """Extract all enum definitions from the source file"""
        for match in self.enum_block_regex.finditer(self.content):
            enum_block = match.group(1)
            start = match.start()
            end = match.end()
            
            # Extract the enum name
            enum_name_match = self.enum_regex.search(enum_block)
            if enum_name_match:
                enum_name = enum_name_match.group(1)
                
                # Clean up the enum block to ensure proper formatting
                # Fix common issues like extra semicolons
                enum_block = re.sub(r'}\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*;E;', r'} \1;', enum_block)
                enum_block = re.sub(r'}\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*;[^;]*;', r'} \1;', enum_block)
                
                # Extract any dependencies from the enum content
                deps = self._extract_dependencies(enum_block)
                
                self.enums.append(Element(enum_name, 'enum', enum_block, start, end, deps))
                
                # Add to symbol map
                self._add_to_symbol_map(enum_name, 'enum')
        
    def extract_conditionals(self):
        """Extract preprocessor conditionals that should be preserved"""
        # First, find all preprocessor directive lines
        directive_lines = []
        for i, line in enumerate(self.content.splitlines()):
            stripped = line.strip()
            # Skip standalone directives that aren't part of proper preprocessor syntax
            if stripped == 'endif' or stripped == 'else' or stripped.startswith('else if'):
                continue
                
            # More comprehensive detection of preprocessor directives
            if (stripped.startswith('#if') or 
                stripped.startswith('#else') or 
                stripped.startswith('#elif') or 
                stripped.startswith('#endif') or
                stripped.startswith('#ifdef') or
                stripped.startswith('#ifndef')):
                directive_lines.append((i, stripped))
        
        # Now process the directives to build complete conditional blocks
        i = 0
        while i < len(directive_lines):
            line_num, directive = directive_lines[i]
            
            # Only process if this is the start of a conditional block
            if directive.startswith('#if'):
                # Find the matching #endif and all intermediate #else/#elif
                stack = 1  # Track nested conditionals
                start_line = line_num
                end_line = None
                block_lines = []
                
                # Add the starting directive
                block_lines.append((line_num, directive))
                
                j = i + 1
                while j < len(directive_lines) and stack > 0:
                    next_line_num, next_directive = directive_lines[j]
                    
                    # More comprehensive detection of opening directives
                    if (next_directive.startswith('#if') or 
                        next_directive.startswith('#ifdef') or 
                        next_directive.startswith('#ifndef')):
                        stack += 1
                    elif next_directive.startswith('#endif'):
                        stack -= 1
                        
                    # Add this directive to our block
                    block_lines.append((next_line_num, next_directive))
                    
                    if stack == 0:
                        end_line = next_line_num
                        break
                        
                    j += 1
                
                # If we found a complete block
                if end_line is not None:
                    # Extract the content between start_line and end_line
                    # Include one line before and after for context if possible
                    context_start = max(0, start_line - 1)
                    context_end = min(len(self.content.splitlines()) - 1, end_line + 1)
                    content_lines = self.content.splitlines()[context_start:context_end+1]
                    content = '\n'.join(content_lines)
                    
                    # Calculate start and end positions in the original content
                    start = len('\n'.join(self.content.splitlines()[:context_start]))
                    end = len('\n'.join(self.content.splitlines()[:context_end+1]))
                    
                    # Extract dependencies from the conditional content
                    deps = self._extract_dependencies(content)
                    
                    # Check if this is a multi-part conditional (#if/#elif/#else chain)
                    is_multipart = False
                    for _, line_directive in block_lines:
                        if line_directive.startswith('#elif') or line_directive.startswith('#else'):
                            is_multipart = True
                            break
                    
                    # Create a name that helps identify related conditionals
                    if is_multipart:
                        # For multi-part conditionals, use a name that includes the condition
                        condition = directive.split(' ', 1)[1] if ' ' in directive else 'condition'
                        name = f"multipart_conditional_{condition}"
                    else:
                        name = 'conditional'
                    
                    # Add the conditional block
                    self.conditionals.append(Element(name, 'conditional', content, start, end, deps))
                    
                    # Skip past this block
                    i = j + 1
                else:
                    # Unterminated conditional - add a warning and fix it
                    print(f"Warning: Unterminated conditional starting at line {start_line+1}: {directive}")
                    
                    # Extract what we have and add missing #endif
                    content_lines = self.content.splitlines()[start_line:]
                    content = '\n'.join(content_lines)
                    content += "\n#endif /* Auto-added to fix unterminated conditional */\n"
                    
                    # Calculate positions
                    start = len('\n'.join(self.content.splitlines()[:start_line]))
                    end = len(self.content)
                    
                    # Extract dependencies
                    deps = self._extract_dependencies(content)
                    
                    # Add the fixed conditional block
                    self.conditionals.append(Element('conditional_fixed', 'conditional', content, start, end, deps))
                    
                    # Skip to the end
                    i = len(directive_lines)
            else:
                # Not the start of a block, move to next directive
                i += 1

    def extract_functions(self):
        """Extract all functions from the source file with timeout protection"""
        for match in self.function_regex.finditer(self.content):
            func_name = match.group(1)
            start = match.start()

            # Find end of function (matching closing brace considering nesting)
            opening_braces = 1
            pos = match.end()
            max_chars_to_check = min(100000, len(self.content) - pos)  # Safety limit
            chars_checked = 0

            while opening_braces > 0 and pos < len(self.content) and chars_checked < max_chars_to_check:
                if self.content[pos] == '{':
                    opening_braces += 1
                elif self.content[pos] == '}':
                    opening_braces -= 1
                pos += 1
                chars_checked += 1

            # Check if we exited due to safety limit
            if opening_braces > 0 and chars_checked >= max_chars_to_check:
                print(f"Warning: Could not find end of function {func_name} within safety limit. Skipping.")
                continue

            if opening_braces == 0:
                func_content = self.content[start:pos].strip()

                # Extract any dependencies from the function content
                deps = self._extract_dependencies(func_content)

                self.functions.append(Element(func_name, 'function', func_content, start, pos, deps))

                # Add to symbol map
                self._add_to_symbol_map(func_name, 'function')
        
    def extract_structs(self):
        """Extract all structs from the source file"""
        for match in self.struct_regex.finditer(self.content):
            struct_name = match.group(1) or match.group(2)
            if struct_name:
                start = match.start()
                end = match.end()
                struct_content = self.content[start:end].strip()
                
                # Extract any dependencies from the struct content
                deps = self._extract_dependencies(struct_content)
                
                self.structs.append(Element(struct_name, 'struct', struct_content, start, end, deps))
                
                # Add to symbol map
                self._add_to_symbol_map(struct_name, 'struct')
        
    def extract_globals(self):
        """Extract all global variables from the source file"""
        for match in self.global_regex.finditer(self.content):
            global_name = match.group(1)
            if global_name:
                # Find end of global definition (semicolon)
                start = match.start()
                line_end = self.content.find(';', match.end())
                if line_end != -1:
                    end = line_end + 1
                    global_content = self.content[start:end].strip()
                    
                    # Skip if it looks like a function forward declaration
                    if not '(' in global_content or not ')' in global_content:
                        # Extract any dependencies
                        deps = self._extract_dependencies(global_content)
                        
                        self.globals.append(Element(global_name, 'global', global_content, start, end, deps))
                        
                        # Add to symbol map
                        self._add_to_symbol_map(global_name, 'global')
        
    def extract_typedefs(self):
        """Extract all typedefs from the source file"""
        for match in self.typedef_regex.finditer(self.content):
            src_type = match.group(1)
            typedef_name = match.group(2)
            if typedef_name:
                start = match.start()
                end = match.end()
                typedef_content = self.content[start:end].strip()
                
                # Extract any dependencies
                deps = self._extract_dependencies(typedef_content)
                
                # Add explicit dependency on the source type
                deps.add(src_type.strip())
                
                self.typedefs.append(Element(typedef_name, 'typedef', typedef_content, start, end, deps))
                
                # Add to symbol map
                self._add_to_symbol_map(typedef_name, 'typedef')
        
    def extract_macros(self):
        """Extract all macros from the source file with improved multi-line and function-like macro handling"""
        # First, find all #define directives
        define_pattern = re.compile(r'(#define\s+([a-zA-Z_][a-zA-Z0-9_]*))(?:\(([^)]*)\))?', re.MULTILINE)
        
        for match in define_pattern.finditer(self.content):
            full_define = match.group(1)
            macro_name = match.group(2)
            params = match.group(3)  # This will be None for non-function-like macros
            
            if macro_name:
                # Find the start of the macro definition
                start = match.start()
                
                # Find the end of the macro definition (handling multi-line macros)
                lines = self.content[match.end():].split('\n')
                end_pos = match.end()
                line_count = 0
                
                # Process each line to find the end of the macro
                for i, line in enumerate(lines):
                    line_count += 1
                    line_length = len(line) + 1  # +1 for the newline
                    
                    # Check if this line ends with a backslash (continuation)
                    if line.strip().endswith('\\'):
                        end_pos += line_length
                        continue
                    else:
                        # This is the last line of the macro
                        end_pos += line_length
                        break
                
                # Extract the complete macro content
                macro_content = self.content[start:end_pos].strip()
                
                # Skip if this is just a macro reference, not a definition
                if not re.search(r'#define\s+' + re.escape(macro_name) + r'\b', macro_content):
                    continue
                
                # Determine if this is a function-like macro
                is_function_like = params is not None
                
                # Extract dependencies, with special handling for function-like macros
                deps = self._extract_macro_dependencies(macro_content, is_function_like, params)
                
                # Create a more descriptive name for function-like macros
                element_name = macro_name
                if is_function_like:
                    element_name = f"{macro_name}({params or ''})"
                
                # Add the macro to our list
                self.macros.append(Element(macro_name, 'macro', macro_content, start, end_pos, deps))
                
                # Add to symbol map with additional metadata
                self.symbol_map[macro_name] = {
                    'type': 'macro',
                    'is_function_like': is_function_like,
                    'used_in': set()
                }
    
    def _extract_macro_dependencies(self, content, is_function_like=False, params=None):
        """Extract dependencies specifically for macros, with special handling for function-like macros"""
        deps = set()
        
        # For function-like macros, we need to exclude the parameter names from dependencies
        param_names = set()
        if is_function_like and params:
            param_names = {p.strip() for p in params.split(',') if p.strip()}
        
        # Find all potential macro references
        # This regex finds identifiers that might be macros
        macro_refs = re.findall(r'(?<![a-zA-Z0-9_])([a-zA-Z_][a-zA-Z0-9_]*)(?!\s*\(|\s*=|\s*\{)', content)
        
        for ref in macro_refs:
            # Skip if this is a parameter name
            if ref in param_names:
                continue
                
            # Skip common keywords
            if ref in {'if', 'else', 'while', 'for', 'return', 'break', 'continue', 
                      'static', 'const', 'void', 'int', 'float', 'double', 'char',
                      'unsigned', 'signed', 'typedef', 'struct', 'enum', 'union',
                      'extern', 'sizeof', 'NULL', 'size_t', 'define'}:
                continue
                
            # Add as a potential dependency
            deps.add(ref)
        
        # Also check for function-like macro calls
        func_macro_calls = re.findall(r'([a-zA-Z_][a-zA-Z0-9_]*)\s*\(', content)
        for call in func_macro_calls:
            if call != 'define':  # Skip the #define itself
                deps.add(call)
        
        return deps
    
    def _extract_dependencies(self, content):
        """Extract symbol dependencies from a piece of code"""
        deps = set()
        
        # Extract all words that might be symbols
        words = re.findall(r'\b([a-zA-Z_][a-zA-Z0-9_]*)\b', content)
        
        # Filter out common keywords and include only known symbols
        keywords = {'if', 'else', 'while', 'for', 'return', 'break', 'continue', 
                   'static', 'const', 'void', 'int', 'float', 'double', 'char',
                   'unsigned', 'signed', 'typedef', 'struct', 'enum', 'union',
                   'extern', 'sizeof', 'NULL', 'size_t'}
        
        for word in words:
            if word not in keywords:
                deps.add(word)
                
        return deps
    
    def _add_to_symbol_map(self, symbol, symbol_type):
        """Add a symbol to the symbol map for dependency tracking"""
        self.symbol_map[symbol] = {
            'type': symbol_type,
            'used_in': set()
        }
    
    def map_symbols_to_components(self):
        """Map all symbols to their target component files"""
        print("\nMapping symbols to components...")
        
        # Map functions
        self._map_functions()
        
        # Map structs
        self._map_structs()
        
        # Map enums
        self._map_enums()
        
        # Map typedefs
        self._map_typedefs()
        
        # Map globals
        self._map_globals()
        
        # Map macros
        self._map_macros()
        
        # Track module dependencies based on symbol usage
        self._analyze_module_dependencies()
        
        # Assign conditionals to appropriate modules
        self._map_conditionals()
        
        # Sort all elements by their original position
        for component in self.output_files:
            self.output_files[component].sort(key=lambda x: x.start)
        
        # Print component statistics
        print("\nComponent mapping complete:")
        for component, elements in sorted(self.output_files.items()):
            element_counts = Counter(elem.type for elem in elements)
            print(f"  {component}: {len(elements)} elements ({', '.join(f'{count} {t}' for t, count in element_counts.items())})")
    
    def _map_functions(self):
        """Map all functions to their target components"""
        for func in self.functions:
            # Find the appropriate component for this function
            component = self._determine_function_component(func.name, func.content)
            
            # Add to the component's elements
            self.output_files[component].append(func)
            
            # Track which component contains this symbol
            if func.name in self.symbol_map:
                self.symbol_map[func.name]['component'] = component
    
    def _map_structs(self):
        """Map all structs to their target components"""
        for struct in self.structs:
            # Find the appropriate component for this struct
            component = self._determine_struct_component(struct.name, struct.content)
            
            # Add to the component's elements
            self.output_files[component].append(struct)
            
            # Track which component contains this symbol
            if struct.name in self.symbol_map:
                self.symbol_map[struct.name]['component'] = component
    
    def _map_enums(self):
        """Map all enums to their target components"""
        for enum in self.enums:
            # Check if this is a common enum that needs to be in common.h
            if enum.name in COMMON_ENUMS:
                component = 'common'
            else:
                # Find the appropriate component
                component = self._determine_enum_component(enum.name, enum.content)
            
            # Add to the component's elements
            self.output_files[component].append(enum)
            
            # Track which component contains this symbol
            if enum.name in self.symbol_map:
                self.symbol_map[enum.name]['component'] = component
    
    def _map_typedefs(self):
        """Map all typedefs to their target components"""
        for typedef in self.typedefs:
            # Determine the component based on the typedef name
            if typedef.name.endswith('_layer'):
                component = 'nn_types'
            elif typedef.name in COMMON_TYPES:
                component = 'common'
            else:
                component = self._determine_typedef_component(typedef.name, typedef.content)
            
            # Add to the component's elements
            self.output_files[component].append(typedef)
            
            # Track which component contains this symbol
            if typedef.name in self.symbol_map:
                self.symbol_map[typedef.name]['component'] = component
    
    def _map_globals(self):
        """Map all globals to their target components"""
        for global_var in self.globals:
            # Determine the component
            component = self._determine_global_component(global_var.name, global_var.content)
            
            # Add to the component's elements
            self.output_files[component].append(global_var)
            
            # Track which component contains this symbol
            if global_var.name in self.symbol_map:
                self.symbol_map[global_var.name]['component'] = component
    
    def _map_macros(self):
        """Map all macros to their target components"""
        for macro in self.macros:
            # Check if this is a required constant for common header
            if macro.name in REQUIRED_CONSTANTS:
                component = 'common'
            else:
                # Determine component based on macro name
                component = self._determine_macro_component(macro.name, macro.content)
            
            # Add to the component's elements
            self.output_files[component].append(macro)
            
            # Track which component contains this symbol
            if macro.name in self.symbol_map:
                self.symbol_map[macro.name]['component'] = component
    
    def _map_conditionals(self):
        """Map conditionals to appropriate components based on their content"""
        # Group conditionals by their preprocessor directive patterns
        directive_patterns = {}
        
        # First, identify all #if/#ifdef/#ifndef blocks and their nested conditionals
        conditional_groups = self._group_related_conditionals()
        
        # Now process each group of related conditionals
        for group_id, conditionals in conditional_groups.items():
            # Determine the best component for this group
            component_votes = Counter()
            
            # Check for specific patterns that should always go to certain components
            forced_component = None
            
            for conditional in conditionals:
                content = conditional.content
                
                # Check for platform-specific code that should always go to common
                if re.search(r'#if\s+defined\s*\(\s*(_WIN32|__APPLE__|__linux__|_MSC_VER)\s*\)', content):
                    forced_component = 'common'
                    break
                
                # Check for OS-specific code
                if re.search(r'#if\s+defined\s*\(\s*(OS_WIN|OS_UNIX|OS_OTHER)\s*\)', content):
                    forced_component = 'common'
                    break
                
                # Check for macro-related conditionals
                if re.search(r'#if(n)?def\s+([a-zA-Z_][a-zA-Z0-9_]*)', content):
                    # Extract the macro name being checked
                    macro_match = re.search(r'#if(n)?def\s+([a-zA-Z_][a-zA-Z0-9_]*)', content)
                    if macro_match:
                        macro_name = macro_match.group(2)
                        # Check if this macro is in our symbol map
                        if macro_name in self.symbol_map and 'component' in self.symbol_map[macro_name]:
                            # Use the same component as the macro
                            forced_component = self.symbol_map[macro_name]['component']
                            break
                
                # Check which symbols are used in this conditional
                for symbol, info in self.symbol_map.items():
                    if symbol in content and 'component' in info:
                        component_votes[info['component']] += 1
            
            # Determine best component based on symbol usage or forced component
            if forced_component:
                component = forced_component
            elif component_votes:
                component = component_votes.most_common(1)[0][0]
            else:
                component = 'common'  # Default
            
            # Add all related conditionals to the same component
            for conditional in conditionals:
                self.output_files[component].append(conditional)
    
    def _group_related_conditionals(self):
        """Group related conditionals that should stay together"""
        # Use a dictionary to track groups of related conditionals
        conditional_groups = {}
        group_id = 0
        
        # First, group by name (which includes condition for multi-part conditionals)
        name_groups = defaultdict(list)
        for conditional in self.conditionals:
            name_groups[conditional.name].append(conditional)
        
        # Create groups from the name groups
        for name, conditionals in name_groups.items():
            if name.startswith('multipart_conditional_'):
                # These are already grouped by name during extraction
                conditional_groups[group_id] = conditionals
                group_id += 1
            else:
                # For regular conditionals, we need to check for relationships
                for conditional in conditionals:
                    # Check if this conditional is already in a group
                    if any(conditional in group for group in conditional_groups.values()):
                        continue
                    
                    # Create a new group for this conditional
                    conditional_groups[group_id] = [conditional]
                    group_id += 1
                    
                    # Find related conditionals based on proximity and content similarity
                    content = conditional.content
                    first_line = content.strip().split('\n')[0].strip()
                    
                    for other in conditionals:
                        if other == conditional or any(other in group for group in conditional_groups.values()):
                            continue
                        
                        # Check if they're close to each other in the source
                        if abs(conditional.start - other.start) < 1000:
                            other_content = other.content
                            other_first_line = other_content.strip().split('\n')[0].strip()
                            
                            # Check for similar conditions
                            if self._are_conditions_similar(first_line, other_first_line):
                                conditional_groups[group_id-1].append(other)
        
        return conditional_groups
    
    def _are_conditions_similar(self, condition1, condition2):
        """Check if two preprocessor conditions are similar enough to be related"""
        # Normalize the conditions
        c1 = re.sub(r'#if\w*\s+', '', condition1)
        c2 = re.sub(r'#if\w*\s+', '', condition2)
        
        # Check for exact match
        if c1 == c2:
            return True
        
        # Check for common patterns
        # Platform checks
        platforms = ['_WIN32', '__APPLE__', '__linux__', '_MSC_VER', 'OS_WIN', 'OS_UNIX', 'OS_OTHER']
        platform_pattern1 = any(platform in c1 for platform in platforms)
        platform_pattern2 = any(platform in c2 for platform in platforms)
        
        if platform_pattern1 and platform_pattern2:
            return True
        
        # Feature checks
        features = ['HAVE_', 'USE_', 'ENABLE_', 'CONFIG_', 'WITH_']
        feature_pattern1 = any(c1.startswith(feature) for feature in features)
        feature_pattern2 = any(c2.startswith(feature) for feature in features)
        
        if feature_pattern1 and feature_pattern2:
            return True
        
        # Check for negated versions of the same condition
        if c1.startswith('!') and c1[1:] == c2:
            return True
        if c2.startswith('!') and c2[1:] == c1:
            return True
        
        return False
    
    def _determine_function_component(self, func_name, content):
        """Determine which component a function belongs to"""
        # Neural network related functions
        if func_name.startswith('forward_') or func_name.startswith('backward_'):
            if 'softmax' in func_name:
                return 'softmax_impl'
            elif 'batchnorm' in func_name:
                return 'batchnorm_impl'
            elif 'connected' in func_name:
                return 'connected_impl'
            elif 'convolutional' in func_name:
                return 'convolutional'
            elif 'local' in func_name:
                return 'local_layer'
            elif 'cost' in func_name:
                return 'cost_layer'
            elif 'route' in func_name:
                return 'route_layer'
            else:
                return 'nn_utils'
        
        # Layer creation functions
        if func_name.startswith('make_'):
            if 'softmax' in func_name:
                return 'softmax_impl'
            elif 'batchnorm' in func_name:
                return 'batchnorm_impl'
            elif 'connected' in func_name:
                return 'connected_impl'
            elif 'local' in func_name:
                return 'local_layer'
            elif 'cost' in func_name:
                return 'cost_layer'
            elif 'route' in func_name:
                return 'route_layer'
            else:
                return 'nn_utils'
        
        # Activation functions
        if any(func_name.startswith(act) for act in ['relu_', 'logistic_', 'tanh_', 'elu_', 'leaky_', 
                                                    'activate', 'gradient', 'stair_', 'hardtan_',
                                                    'lhtan_', 'relie_', 'ramp_', 'plse_', 'loggy_']):
            return 'activation'
            
        # Data structure functions
        if func_name.startswith('SyBlob') or func_name.startswith('SySet') or func_name.startswith('SyString'):
            return 'data_structures'
            
        # File system functions
        if func_name.startswith('UnixVfs') or func_name.startswith('WinVfs') or func_name.startswith('UnixDir') or func_name.startswith('WinDir'):
            return 'vfs'
            
        # Box utility functions
        if 'box' in func_name.lower():
            return 'box_utils'
            
        # Image related functions
        if any(x in func_name.lower() for x in ['img', 'image']):
            return 'img_utils'
            
        # CPU math functions
        if any(func_name.endswith(x) for x in ['_cpu']):
            return 'cpu_utils'
            
        # RNN related functions
        if 'rnn' in func_name.lower() or 'gru' in func_name.lower() or 'lstm' in func_name.lower():
            return 'rnn'
            
        # CNN related functions
        if 'cnn' in func_name.lower():
            return 'cnn'
            
        # Detection related
        if 'detect' in func_name.lower() or 'realnet' in func_name.lower():
            return 'detection'
            
        # Config parsing
        if any(func_name.startswith(x) for x in ['parse_', 'option_', 'load_']):
            return 'cfg_parser'
        
        # Default to nn_utils if nothing else matched
        return 'nn_utils'
    
    def _determine_struct_component(self, struct_name, content):
        """Determine which component a struct belongs to"""
        # Neural network related structs
        if struct_name in ['layer', 'network', 'network_state', 'size_params']:
            return 'nn_types'
            
        # Data structure related structs
        if struct_name in ['SyBlob', 'SySet', 'SyString']:
            return 'data_structures'
            
        # Box related structs
        if struct_name == 'box' or 'box' in struct_name.lower():
            return 'box_utils'
            
        # Image related structs
        if 'img' in struct_name.lower() or 'image' in struct_name.lower():
            return 'img_utils'
            
        # VFS related structs
        if 'vfs' in struct_name.lower():
            return 'vfs'
            
        # CNN related structs
        if 'cnn' in struct_name.lower():
            return 'cnn'
            
        # Detection related structs
        if 'detect' in struct_name.lower() or 'realnet' in struct_name.lower():
            return 'detection'
            
        # Config/parser related structs
        if any(x in struct_name.lower() for x in ['config', 'list', 'section', 'node']):
            return 'cfg_parser'
            
        # Default to data_structures if nothing else matched
        return 'data_structures'

    def _determine_global_component(self, global_name, content):
        """Determine which component a global variable belongs to"""
        # Neural network related globals
        if any(x in global_name for x in ['weights', 'biases', 'scales', 'rolling', 'adam']):
            return 'nn_utils'

        # Activation related globals
        if any(x in global_name for x in ['activate', 'gradient']):
            return 'activation'

        # Image processing related globals
        if any(x in global_name.lower() for x in ['img', 'image', 'pixel']):
            return 'img_utils'

        # Detection related globals
        if any(x in global_name.lower() for x in ['detect', 'box', 'anchor']):
            return 'detection'

        # File system related globals
        if any(x in global_name.lower() for x in ['file', 'dir', 'path']):
            return 'vfs'

        # CNN related globals
        if 'cnn' in global_name.lower():
            return 'cnn'

        # Default to common if nothing else matched
        return 'common'

    def _determine_enum_component(self, enum_name, content):
        """Determine which component an enum belongs to"""
        # Neural network related enums
        if any(x in enum_name for x in ['ACTIVATION', 'COST_TYPE', 'layer_type']):
            return 'nn_types'

        # CNN related enums
        if 'CNN' in enum_name:
            return 'cnn'

        # Detection related enums
        if any(x in enum_name for x in ['REALNET', 'TR_SAMPLE']):
            return 'detection'

        # Default to common if nothing else matched
        return 'common'

    def _determine_typedef_component(self, typedef_name, content):
        """Determine which component a typedef belongs to"""
        # Neural network related typedefs
        if any(x in typedef_name for x in ['network', 'layer', 'cost', 'activation']):
            return 'nn_types'

        # Data structure related typedefs
        if any(x in typedef_name for x in ['Sy', 'blob', 'set', 'string']):
            return 'data_structures'

        # Image processing related typedefs
        if any(x in typedef_name.lower() for x in ['img', 'image', 'ipl']):
            return 'img_utils'

        # Detection related typedefs
        if any(x in typedef_name.lower() for x in ['detect', 'box', 'pts']):
            return 'detection'

        # File system related typedefs
        if any(x in typedef_name.lower() for x in ['vfs', 'file', 'dir']):
            return 'vfs'

        # Default to common if nothing else matched
        return 'common'

    def _determine_macro_component(self, macro_name, content):
        """Determine which component a macro belongs to"""
        # Common constants
        if macro_name in REQUIRED_CONSTANTS:
            return 'common'

        # Neural network related macros
        if any(x in macro_name for x in ['LAYER', 'ACTIVATION', 'WEIGHT']):
            return 'nn_types'

        # Image processing related macros
        if any(x in macro_name.lower() for x in ['img', 'image', 'pixel']):
            return 'img_utils'

        # Detection related macros
        if any(x in macro_name for x in ['BOX', 'DETECT', 'CNN']):
            return 'detection'

        # File system related macros
        if any(x in macro_name for x in ['FILE', 'DIR', 'PATH']):
            return 'vfs'

        # Default to common if nothing else matched
        return 'common'

    def _analyze_module_dependencies(self):
        """Analyze dependencies between modules based on symbol usage"""
        # For each symbol, check which components use it
        for symbol, info in self.symbol_map.items():
            if 'component' in info:
                source_component = info['component']

                # For each component that uses this symbol
                for component in info['used_in']:
                    # Skip self-dependencies
                    if component != source_component:
                        # Add dependency
                        self.module_deps[component].add(source_component)

        # Print module dependencies
        print("\nModule dependencies:")
        for module, deps in sorted(self.module_deps.items()):
            print(f"  {module} depends on: {', '.join(sorted(deps))}")


    def _get_necessary_includes(self, elements):
        """Determine which other modules need to be included"""
        includes = set()

        # Define type-to-module mappings
        type_mappings = {
            'sod_cnn': 'cnn',
            'sod_img': 'img_utils',
            'sod_box': 'box_utils',
            'box': 'box_utils',
            'network': 'nn_types',
            'layer': 'nn_types',
            'tree': 'nn_types',
            'ACTIVATION': 'activation',
            'SOD_CNN_LAYER_TYPE': 'nn_types',
            'learning_rate_policy': 'nn_types',
            'COST_TYPE': 'nn_types',
            'SyBlob': 'data_structures',
            'SySet': 'data_structures',
            'SyString': 'data_structures',
            'sod_vfs': 'vfs',
            'softmax_layer': 'softmax_impl',
            'local_layer': 'local_layer',
            'connected_layer': 'connected_impl',
            'convolutional_layer': 'convolutional',
            'cost_layer': 'cost_layer',
            'route_layer': 'route_layer'
        }

        # Common function prefixes and their modules
        function_prefixes = {
            'forward_softmax': 'softmax_impl',
            'backward_softmax': 'softmax_impl',
            'forward_batchnorm': 'batchnorm_impl',
            'backward_batchnorm': 'batchnorm_impl',
            'forward_connected': 'connected_impl',
            'backward_connected': 'connected_impl',
            'forward_convolutional': 'convolutional',
            'backward_convolutional': 'convolutional',
            'forward_cost': 'cost_layer',
            'backward_cost': 'cost_layer',
            'forward_local': 'local_layer',
            'backward_local': 'local_layer',
            'forward_route': 'route_layer',
            'backward_route': 'route_layer',
            'activate': 'activation',
            'gradient': 'activation',
            'SyBlob': 'data_structures',
            'SySet': 'data_structures',
            'SyString': 'data_structures'
        }

        # For each element, check for dependencies on other modules
        for elem in elements:
            content = elem.content

            # Check for type references
            for type_name, module in type_mappings.items():
                if type_name in content and module != 'common':
                    includes.add(module)

            # Check for function call patterns
            for prefix, module in function_prefixes.items():
                if prefix in content and module != 'common':
                    includes.add(module)

            # Also check dependencies directly from the element
            for dep in elem.deps:
                # Check if this dependency is mapped to a module
                for type_name, module in type_mappings.items():
                    if dep == type_name and module != 'common':
                        includes.add(module)

        return includes

    def _extract_module_definitions(self, module_name):
        """Extract enum definitions for a specific module"""
        definitions = ""

        # Find all enum elements in this module
        for elem in self.output_files.get(module_name, []):
            if elem.type == 'enum':
                definitions += elem.content + "\n\n"

        return definitions

    def create_output_files(self):
        """Create all output files with their assigned elements"""
        # First, extract definitions for specific modules
        nn_types_definitions = self._extract_module_definitions('nn_types')
        activation_definitions = self._extract_module_definitions('activation')

        # Create a common header file first
        self._create_common_header()

        for file_key, elements in self.output_files.items():
            # Two files: .c implementation and .h header
            c_path = os.path.join(self.src_dir, f'sod_{file_key}.c')
            h_path = os.path.join(self.include_dir, f'sod_{file_key}.h')

            # Prepare header content
            header_content = f"""
/* 
 * sod_{file_key}.h - Part of the SOD library
 * Generated from the original monolithic code
 */

#ifndef SOD_{file_key.upper()}_H__
#define SOD_{file_key.upper()}_H__

#include "sod/sod_common.h"

"""
            # Add specialized type definitions for specific modules that were extracted
            if file_key == 'nn_types' and nn_types_definitions:
                header_content += nn_types_definitions + "\n\n"
            elif file_key == 'activation' and activation_definitions:
                header_content += activation_definitions + "\n\n"

            # Extract header declarations
            header_elements = []
            for elem in elements:
                if elem.type in ['struct', 'typedef', 'macro', 'enum']:
                    # Skip enums that are already defined in NN_TYPES_DEFINITIONS or ACTIVATION_DEFINITIONS
                    if elem.type == 'enum':
                        if file_key == 'nn_types' and elem.name in ['SOD_CNN_LAYER_TYPE', 'learning_rate_policy', 'COST_TYPE']:
                            continue
                        elif file_key == 'activation' and elem.name == 'ACTIVATION':
                            continue
                    header_elements.append(elem)
                elif elem.type == 'function':
                    # Extract function declaration
                    decl_end = elem.content.find('{')
                    if decl_end != -1:
                        decl = elem.content[:decl_end].strip() + ';'
                        # Skip static functions in headers
                        if not decl.startswith('static'):
                            header_elements.append(Element(elem.name, 'declaration', decl, elem.start, elem.start + len(decl), set()))
                elif elem.type == 'conditional':
                    # For conditionals in headers, we need to extract only declarations, not implementations
                    content = elem.content
                    
                    # Check if conditional contains declarations that should go in header
                    if re.search(self.struct_regex, content) or re.search(self.enum_regex, content) or re.search(self.typedef_regex, content):
                        # Extract only the declarations from the conditional
                        header_content = []
                        
                        # Extract the preprocessor directives
                        directive_lines = []
                        for line in content.splitlines():
                            if line.strip().startswith('#'):
                                directive_lines.append(line)
                        
                        # Extract struct, enum, and typedef declarations
                        for match in self.struct_regex.finditer(content):
                            header_content.append(content[match.start():match.end()])
                        
                        for match in self.enum_regex.finditer(content):
                            header_content.append(content[match.start():match.end()])
                        
                        for match in self.typedef_regex.finditer(content):
                            header_content.append(content[match.start():match.end()])
                        
                        # Extract function declarations (not implementations)
                        for match in re.finditer(r'(?:SOD_APIEXPORT\s+)?(?:[a-zA-Z_][a-zA-Z0-9_*\s]+?\s+)([a-zA-Z_][a-zA-Z0-9_]*)\s*\([^{;]*\)\s*;', content):
                            header_content.append(content[match.start():match.end()])
                        
                        # If we found declarations, create a new conditional with just those declarations
                        if header_content:
                            # Start with the opening directive
                            new_content = []
                            in_directive = False
                            
                            for line in content.splitlines():
                                if line.strip().startswith('#'):
                                    new_content.append(line)
                                    if line.strip().startswith('#if') or line.strip().startswith('#ifdef') or line.strip().startswith('#ifndef'):
                                        in_directive = True
                                    elif line.strip().startswith('#endif'):
                                        in_directive = False
                                elif in_directive and any(decl in line for decl in header_content):
                                    new_content.append(line)
                            
                            # Create a new conditional element with just the declarations
                            new_conditional = Element(
                                elem.name,
                                'conditional',
                                '\n'.join(new_content),
                                elem.start,
                                elem.end,
                                elem.deps
                            )
                            header_elements.append(new_conditional)

            # Make sure all conditional blocks are properly closed
            # For the header elements loop
            for i, elem in enumerate(header_elements):
                if elem.type == 'conditional':
                    content = elem.content
                    # Count #if, #ifdef, #ifndef directives
                    open_directives = len(re.findall(r'#if\b|#ifdef\b|#ifndef\b', content))
                    # Count #endif directives
                    close_directives = len(re.findall(r'#endif\b', content))
                    # Add missing #endif directives
                    if open_directives > close_directives:
                        for _ in range(open_directives - close_directives):
                            content += "\n#endif /* End of condition */\n"
                        # Add deps parameter here
                        header_elements[i] = Element(elem.name, elem.type, content, elem.start, elem.end, elem.deps)

            # Add elements to header
            for elem in sorted(header_elements, key=lambda x: x.start):
                header_content += elem.content + '\n\n'

            header_content += f"\n#endif /* SOD_{file_key.upper()}_H__ */\n"

            # Prepare implementation content
            impl_content = f"""
/* 
 * sod_{file_key}.c - Part of the SOD library
 * Generated from the original monolithic code
 */

"""
            # Add standard headers
            for header in STANDARD_HEADERS:
                impl_content += f"#include {header}\n"

            impl_content += f"\n#include \"sod_{file_key}.h\"\n"

            # Add any needed additional includes
            includes = self._get_necessary_includes(elements)
            # Always include common types
            if 'common' not in includes and file_key != 'common':
                includes.add('common')
            # Special dependencies
            if file_key in ['cnn', 'detection', 'box_utils']:
                if 'nn_types' not in includes:
                    includes.add('nn_types')
            if file_key in ['activation', 'dropout']:
                if 'nn_types' not in includes:
                    includes.add('nn_types')

            for include in includes:
                if include != file_key:  # Avoid self-include
                    impl_content += f'#include "sod/sod_{include}.h"\n'

            impl_content += "\n"

            # Fix common preprocessing issues
            if file_key == 'common':
                # Ensure all #if/#ifdef have matching #endif
                impl_content += "#ifndef OS_OTHER\n#define OS_OTHER\n#endif\n\n"

            # Add implementation elements
            impl_elements = []
            for elem in elements:
                if elem.type in ['function', 'global'] or (elem.type == 'conditional' and not any(e == elem for e in header_elements)):
                    impl_elements.append(elem)

            # Make sure all conditional blocks are properly closed for implementation too
            # For the implementation elements loop
            for i, elem in enumerate(impl_elements):
                if elem.type == 'conditional':
                    content = elem.content
                    # Count #if, #ifdef, #ifndef directives
                    open_directives = len(re.findall(r'#if\b|#ifdef\b|#ifndef\b', content))
                    # Count #endif directives
                    close_directives = len(re.findall(r'#endif\b', content))
                    # Add missing #endif directives
                    if open_directives > close_directives:
                        for _ in range(open_directives - close_directives):
                            content += "\n#endif /* End of condition */\n"
                        # Add deps parameter here
                        impl_elements[i] = Element(elem.name, elem.type, content, elem.start, elem.end, elem.deps)
                    
                    # Fix stray preprocessor directives
                    # Check for standalone 'endif' or 'else if' blocks
                    lines = content.splitlines()
                    fixed_lines = []
                    for line in lines:
                        # Skip standalone 'endif' lines
                        if line.strip() == 'endif':
                            continue
                        # Fix standalone 'else if' blocks
                        if line.strip().startswith('else if'):
                            continue
                        fixed_lines.append(line)
                    
                    # Update the element with fixed content
                    if len(fixed_lines) < len(lines):
                        fixed_content = '\n'.join(fixed_lines)
                        impl_elements[i] = Element(elem.name, elem.type, fixed_content, elem.start, elem.end, elem.deps)

            # Sort elements by their original position
            for elem in sorted(impl_elements, key=lambda x: x.start):
                # Do one final check for stray preprocessor directives
                if elem.type == 'conditional' or elem.type == 'function':
                    content = elem.content
                    lines = content.splitlines()
                    fixed_lines = []
                    for line in lines:
                        # Skip standalone preprocessor directives
                        if line.strip() == 'endif' or line.strip() == 'else' or line.strip().startswith('else if'):
                            continue
                        fixed_lines.append(line)
                    
                    if len(fixed_lines) < len(lines):
                        content = '\n'.join(fixed_lines)
                        impl_content += content + '\n\n'
                    else:
                        impl_content += elem.content + '\n\n'
                else:
                    impl_content += elem.content + '\n\n'

            # Write files
            with open(c_path, 'w', encoding='utf-8') as f:
                f.write(impl_content)

            with open(h_path, 'w', encoding='utf-8') as f:
                f.write(header_content)

            print(f"Created {file_key} module ({len(elements)} elements)")
            
    def _create_common_header(self):
        """Create the common header file with all required definitions"""
        h_path = os.path.join(self.include_dir, 'sod_common.h')
        
        header_content = """
/* 
 * sod_common.h - Common definitions for the SOD library
 * Generated from the original monolithic code
 */

#ifndef SOD_COMMON_H__
#define SOD_COMMON_H__

/* Standard includes */
"""
        # Add standard headers with platform-specific handling
        for header in STANDARD_HEADERS:
            header_content += f"#include {header}\n"
        
        # Add platform-specific includes
        header_content += """
/* Platform-specific includes */
#if defined(_WIN32) || defined(_MSC_VER)
#define __WINNT__ 1
#include <windows.h>
#elif defined(__APPLE__) || defined(__linux__) || defined(__unix__)
#define __UNIXES__ 1
#include <unistd.h>
#include <sys/types.h>
#endif

/* Define PATH_MAX if not already defined */
#ifndef PATH_MAX
#ifdef _WIN32
#define PATH_MAX 260
#else
#define PATH_MAX 4096
#endif
#endif

"""
            
        # Add API export macro
        header_content += f"\n{SOD_API_EXPORT_MACRO}\n"
        
        # Add required typedefs
        header_content += f"\n{REQUIRED_TYPEDEFS}\n"
        
        # Add required constants
        header_content += "\n/* Required constants */\n"
        for constant in REQUIRED_CONSTANTS:
            # Find the macro definition
            for macro in self.macros:
                if macro.name == constant:
                    header_content += macro.content + "\n"
                    break
        
        # Add forward declarations for common types
        header_content += "\n/* Forward declarations for common types */\n"
        for type_name in COMMON_TYPES:
            header_content += f"typedef struct {type_name} {type_name};\n"
        
        # Add common enums - with careful cleaning
        header_content += "\n/* Common enumerations */\n"
        for enum in self.enums:
            if enum.name in COMMON_ENUMS:
                # Clean up the enum content to ensure proper formatting
                enum_content = enum.content
                enum_content = re.sub(r'}\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*;E;', r'} \1;', enum_content)
                enum_content = re.sub(r'}\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*;[^;]*;', r'} \1;', enum_content)
                header_content += enum_content + "\n\n"
        
        header_content += "\n#endif /* SOD_COMMON_H__ */\n"
        
        # Write the file
        with open(h_path, 'w', encoding='utf-8') as f:
            f.write(header_content)
            
        print(f"Created common header file")


    def create_main_header(self):
        """Create the main SOD header file that includes all component headers"""
        h_path = os.path.join(self.include_dir, '..', 'sod.h')
        
        header_content = """
/* 
 * sod.h - Main header file for the SOD library
 * Generated from the original monolithic code
 */

#ifndef SOD_H__
#define SOD_H__

/* Include all component headers */
#include "sod/sod_common.h"
"""
        
        # Add includes for all components in dependency order
        for component in sorted(self.output_files.keys()):
            if component != 'common':  # common is already included
                header_content += f'#include "sod/sod_{component}.h"\n'
        
        header_content += "\n#endif /* SOD_H__ */\n"
        
        # Write the file
        with open(h_path, 'w', encoding='utf-8') as f:
            f.write(header_content)
            
        print(f"Created main header file: {h_path}")

    def extract_and_process(self):
        """Main method to extract all elements and create output files"""
        print("Starting SOD library splitting process...")

        try:
            # Extract all symbols from the source file
            self.extract_symbols()

            # Map symbols to components
            self.map_symbols_to_components()

            # Create output directories if they don't exist
            os.makedirs(self.src_dir, exist_ok=True)
            os.makedirs(self.include_dir, exist_ok=True)
            os.makedirs(os.path.dirname(os.path.join(self.include_dir, '..')), exist_ok=True)

            # Create all component files
            self.create_output_files()
            
            # Create the main header file
            self.create_main_header()

            print(f"\nSplitting complete! Created {len(self.output_files)} components.")
            
            # Verify the output
            self._verify_output()
            
        except Exception as e:
            print(f"Error during processing: {str(e)}")
            import traceback
            traceback.print_exc()
            
    def _verify_output(self):
        """Verify that all preprocessor directives are properly balanced and fix if needed
        
        Returns:
            bool: True if issues were found, False otherwise
        """
        print("\nVerifying output files...")
        
        issues_found = False
        
        # Check each output file
        for file_key in self.output_files:
            c_path = os.path.join(self.src_dir, f'sod_{file_key}.c')
            h_path = os.path.join(self.include_dir, f'sod_{file_key}.h')
            
            for path in [c_path, h_path]:
                if os.path.exists(path):
                    with open(path, 'r', encoding='utf-8') as f:
                        content = f.read()
                        
                    # Check for balanced preprocessor directives
                    open_directives = len(re.findall(r'#if\b|#ifdef\b|#ifndef\b', content))
                    close_directives = len(re.findall(r'#endif\b', content))
                    
                    if open_directives != close_directives:
                        print(f"Warning: Unbalanced preprocessor directives in {path}")
                        print(f"  Open directives: {open_directives}")
                        print(f"  Close directives: {close_directives}")
                        issues_found = True
                        
                        # Fix the file by adding missing #endif directives
                        if open_directives > close_directives:
                            with open(path, 'a', encoding='utf-8') as f:
                                for _ in range(open_directives - close_directives):
                                    f.write("\n#endif /* Auto-added to balance directives */\n")
                            print(f"  Fixed by adding {open_directives - close_directives} #endif directives")
                        elif close_directives > open_directives:
                            # This is harder to fix automatically - we'd need to remove some #endif directives
                            print(f"  Warning: More #endif directives than #if directives. Manual inspection needed.")
                    
                    # Check for other common issues
                    file_issues = self._check_for_common_issues(path, content)
                    if file_issues:
                        issues_found = True
        
        return issues_found
    
    def _check_for_common_issues(self, path, content):
        """Check for common issues in the output files and fix them if possible"""
        try:
            # Set a timeout for processing large files
            max_processing_time = 10  # seconds
            start_time = time.time()
            
            issues_found = False
            fixed_content = content
            
            # Check for unterminated string literals (with timeout)
            if time.time() - start_time < max_processing_time:
                # Improved regex to find unterminated string literals
                # Look for lines with an odd number of double quotes
                lines = content.split('\n')
                unterminated_lines = []
                
                for i, line in enumerate(lines):
                    # Skip comments
                    if line.strip().startswith('//') or line.strip().startswith('/*'):
                        continue
                        
                    # Count quotes in this line
                    quotes = line.count('"')
                    if quotes % 2 != 0:
                        unterminated_lines.append((i, line))
                
                if unterminated_lines:
                    print(f"Warning: Found {len(unterminated_lines)} potentially unterminated string literals in {path}")
                    issues_found = True
                    
                    # Try to fix unterminated strings by adding closing quotes
                    for i, line in unterminated_lines:
                        if line.count('"') % 2 != 0:
                            # Add a closing quote at the end of the line
                            lines[i] = line + '"'
                    
                    fixed_content = '\n'.join(lines)
            
            # Check for unbalanced braces (with timeout)
            if time.time() - start_time < max_processing_time:
                # More sophisticated brace counting that ignores braces in comments and strings
                lines = fixed_content.split('\n')
                in_comment = False
                brace_balance = 0
                line_balances = []
                
                for i, line in enumerate(lines):
                    j = 0
                    while j < len(line):
                        # Skip string literals
                        if line[j] == '"' and (j == 0 or line[j-1] != '\\'):
                            j += 1
                            while j < len(line) and (line[j] != '"' or line[j-1] == '\\'):
                                j += 1
                            if j < len(line):
                                j += 1  # Skip the closing quote
                            continue
                            
                        # Skip single-line comments
                        if j < len(line) - 1 and line[j:j+2] == '//':
                            break  # Skip rest of line
                            
                        # Handle multi-line comments
                        if j < len(line) - 1 and line[j:j+2] == '/*':
                            in_comment = True
                            j += 2
                            continue
                            
                        if in_comment and j < len(line) - 1 and line[j:j+2] == '*/':
                            in_comment = False
                            j += 2
                            continue
                            
                        if in_comment:
                            j += 1
                            continue
                            
                        # Count braces outside comments and strings
                        if line[j] == '{':
                            brace_balance += 1
                        elif line[j] == '}':
                            brace_balance -= 1
                            
                        j += 1
                    
                    line_balances.append((i, brace_balance))
                
                if brace_balance != 0:
                    print(f"Warning: Unbalanced braces in {path}")
                    print(f"  Brace balance: {brace_balance}")
                    issues_found = True
                    
                    # Try to fix unbalanced braces
                    if brace_balance > 0:
                        # Add missing closing braces
                        fixed_content += "\n" + "}" * brace_balance + " /* Auto-added to balance braces */\n"
                    elif brace_balance < 0:
                        # This is harder to fix - try to identify and comment out extra closing braces
                        # Start from the end and work backwards
                        extra_braces = -brace_balance
                        for i in range(len(lines) - 1, -1, -1):
                            if extra_braces <= 0:
                                break
                                
                            line = lines[i]
                            if '}' in line and not line.strip().startswith('//') and '/*' not in line:
                                # Comment out the last closing brace in this line
                                last_brace = line.rindex('}')
                                lines[i] = line[:last_brace] + '/* Extra closing brace removed */' + line[last_brace+1:]
                                extra_braces -= 1
                                
                        fixed_content = '\n'.join(lines)
            
            # Check for missing semicolons after struct/enum definitions (with timeout)
            if time.time() - start_time < max_processing_time:
                # Improved regex to find struct/enum definitions without semicolons
                struct_enum_defs = re.findall(r'(typedef\s+struct|typedef\s+enum)[^;{]*{[^}]*}\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*(?!;)', fixed_content)
                if struct_enum_defs:
                    print(f"Warning: Found {len(struct_enum_defs)} struct/enum definitions without semicolons in {path}")
                    
                    # Fix missing semicolons
                    for match in struct_enum_defs[:50]:  # Limit to first 50 to avoid excessive processing
                        pattern = r'(' + re.escape(match[0]) + r'[^;{]*{[^}]*}\s*' + re.escape(match[1]) + r')\s*(?!;)'
                        replacement = r'\1;'
                        fixed_content = re.sub(pattern, replacement, fixed_content)
                    
                    issues_found = True
            
            # Fix malformed enum definitions with extra semicolons or 'E;' (with timeout)
            if time.time() - start_time < max_processing_time:
                # First, fix the most problematic patterns that cause compilation errors
                fixed_content = re.sub(r'}\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*;E;', r'} \1;', fixed_content)
                fixed_content = re.sub(r'}\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*;[^;{}\n]*;', r'} \1;', fixed_content)
                
                # More aggressive fixes for malformed enums
                fixed_content = re.sub(r'}\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*;[^{}\n]*?;', r'} \1;', fixed_content)
                
                # Report the fixes
                malformed_enums = re.findall(r'}\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*;E;', content)
                if malformed_enums:
                    print(f"Warning: Found {len(malformed_enums)} malformed enum definitions in {path}")
                    issues_found = True
                
                malformed_enums2 = re.findall(r'}\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*;[^;{}\n]*;', content)
                if malformed_enums2:
                    print(f"Warning: Found {len(malformed_enums2)} enums with multiple semicolons in {path}")
                    issues_found = True
            
            # Fix Windows.h include on non-Windows platforms (with timeout)
            if time.time() - start_time < max_processing_time:
                if 'Windows.h' in fixed_content and not os.name == 'nt':
                    print(f"Warning: Found Windows.h include in {path} on non-Windows platform")
                    fixed_content = re.sub(r'#include\s+<Windows.h>', r'#ifdef _WIN32\n#include <windows.h>\n#endif', fixed_content)
                    issues_found = True
            
            # Check for macro-specific issues (with timeout)
            if time.time() - start_time < max_processing_time:
                fixed_content = self._check_for_macro_issues(path, fixed_content)
            
            # Write the fixed content back to the file if changes were made
            if fixed_content != content:
                print(f"  Fixed issues in {path}")
                with open(path, 'w', encoding='utf-8') as f:
                    f.write(fixed_content)
            
            # Check if we timed out
            if time.time() - start_time >= max_processing_time:
                print(f"Warning: Processing of {path} timed out. Some issues may not have been fixed.")
            
            return issues_found
        except Exception as e:
            print(f"Error while checking for issues in {path}: {str(e)}")
            return False
    
    def _check_for_macro_issues(self, path, content):
        """Check for macro-specific issues in the output files and fix them if possible"""
        try:
            # Set a timeout for processing large files
            max_processing_time = 5  # seconds
            start_time = time.time()
            
            fixed_content = content
            issues_fixed = False
            
            # Check for multi-line macros that might be missing continuation backslashes (with timeout)
            if time.time() - start_time < max_processing_time:
                # Find all #define directives
                lines = fixed_content.splitlines()
                for i, line in enumerate(lines):
                    if line.startswith('#define') and len(line) > 80 and not line.endswith('\\'):
                        print(f"Warning: Long macro definition without continuation in {path}")
                        # Add continuation backslash at position 79
                        lines[i] = line[:79] + ' \\'
                        
                        # If this is a multi-line macro that's missing continuation, 
                        # we need to add the next line as a continuation
                        if i+1 < len(lines) and not lines[i+1].startswith('#'):
                            lines[i+1] = "    " + lines[i+1]
                            issues_fixed = True
                
                if issues_fixed:
                    fixed_content = '\n'.join(lines)
            
            # Check for macro redefinitions (with timeout)
            if time.time() - start_time < max_processing_time:
                # More sophisticated approach to find macro redefinitions
                # that ignores redefinitions inside #ifndef/#endif blocks
                lines = fixed_content.splitlines()
                macro_defs = {}
                in_guard = {}
                directive_stack = []
                
                for i, line in enumerate(lines):
                    # Track preprocessor directive nesting
                    if re.match(r'^\s*#\s*if', line):
                        directive_stack.append(i)
                    elif re.match(r'^\s*#\s*endif', line) and directive_stack:
                        directive_stack.pop()
                        
                    # Find macro definitions
                    macro_match = re.match(r'^\s*#\s*define\s+([a-zA-Z_][a-zA-Z0-9_]*)', line)
                    if macro_match:
                        macro_name = macro_match.group(1)
                        
                        # Check if this is inside an include guard
                        is_guarded = False
                        if directive_stack:
                            guard_line = lines[directive_stack[-1]]
                            if re.search(r'#ifndef\s+' + re.escape(macro_name), guard_line):
                                is_guarded = True
                                in_guard[macro_name] = True
                        
                        # Only track as redefinition if not in a guard
                        if not is_guarded and not in_guard.get(macro_name, False):
                            if macro_name in macro_defs:
                                print(f"Warning: Macro '{macro_name}' is redefined at line {i+1} in {path}")
                                # Comment out the redefinition
                                lines[i] = f"/* Duplicate definition removed: {line} */"
                                issues_fixed = True
                            else:
                                macro_defs[macro_name] = i
                
                if issues_fixed:
                    fixed_content = '\n'.join(lines)
            
            # Check for #undef without corresponding #define (with timeout)
            if time.time() - start_time < max_processing_time:
                # Get all defined macros
                defined_macros = set(re.findall(r'#\s*define\s+([a-zA-Z_][a-zA-Z0-9_]*)', fixed_content))
                
                # Find all undefs
                undef_matches = list(re.finditer(r'#\s*undef\s+([a-zA-Z_][a-zA-Z0-9_]*)', fixed_content))
                
                for match in undef_matches:
                    undef_name = match.group(1)
                    if undef_name not in defined_macros:
                        print(f"Warning: #undef for '{undef_name}' without corresponding #define in {path}")
                        # Comment out the #undef
                        start = match.start()
                        end = match.end()
                        fixed_content = fixed_content[:start] + f"/* Commented out as no matching #define found: {fixed_content[start:end]} */" + fixed_content[end:]
                        issues_fixed = True
            
            # Fix unbalanced preprocessor directives (with timeout)
            if time.time() - start_time < max_processing_time:
                # More sophisticated approach to track and fix preprocessor directive balance
                lines = fixed_content.splitlines()
                directive_stack = []
                unmatched_endifs = []
                
                for i, line in enumerate(lines):
                    stripped = line.strip()
                    
                    # Skip comments
                    if stripped.startswith('//') or stripped.startswith('/*'):
                        continue
                        
                    # Track opening directives
                    if re.match(r'^\s*#\s*(if|ifdef|ifndef)\b', stripped):
                        directive_stack.append((i, stripped))
                    
                    # Track closing directives
                    elif re.match(r'^\s*#\s*endif\b', stripped):
                        if directive_stack:
                            directive_stack.pop()
                        else:
                            unmatched_endifs.append(i)
                
                # Fix unbalanced directives
                if directive_stack or unmatched_endifs:
                    print(f"Warning: Unbalanced preprocessor directives in {path}")
                    print(f"  Missing #endif directives: {len(directive_stack)}")
                    print(f"  Extra #endif directives: {len(unmatched_endifs)}")
                    
                    # Comment out extra #endif directives
                    for i in reversed(unmatched_endifs):  # Process in reverse to avoid index shifting
                        lines[i] = f"/* Extra #endif removed: {lines[i]} */"
                    
                    # Add missing #endif directives
                    if directive_stack:
                        for i, directive in directive_stack:
                            # Extract the condition from the directive for better comments
                            condition = re.sub(r'^\s*#\s*(if|ifdef|ifndef)\s+', '', directive)
                            lines.append(f"#endif /* Auto-added to match {directive} at line {i+1} */")
                    
                    fixed_content = '\n'.join(lines)
                    issues_fixed = True
            
            # Add additional checks for common macro issues
            if time.time() - start_time < max_processing_time:
                # Check for macros with missing parentheses around parameters
                macro_defs = re.findall(r'#define\s+([a-zA-Z_][a-zA-Z0-9_]*)\(([^)]*)\)\s+(.+)', fixed_content)
                for name, params, body in macro_defs:
                    param_list = [p.strip() for p in params.split(',') if p.strip()]
                    for param in param_list:
                        # Look for parameter used without parentheses in arithmetic or logical operations
                        if re.search(r'[+\-*/&|^<>]=?\s*' + re.escape(param) + r'\b|\b' + re.escape(param) + r'\s*[+\-*/&|^<>]=?', body):
                            if not re.search(r'\(' + re.escape(param) + r'\)', body):
                                print(f"Warning: Macro '{name}' may need parentheses around parameter '{param}' in {path}")
                                # This is just a warning, we don't auto-fix this as it requires careful analysis
            
            # Check if we timed out
            if time.time() - start_time >= max_processing_time:
                print(f"Warning: Macro issue checking for {path} timed out. Some issues may not have been fixed.")
            
            if issues_fixed:
                return fixed_content
            else:
                return content
        except Exception as e:
            print(f"Error while checking for macro issues in {path}: {str(e)}")
            return content

    def _create_common_header(self):
        """Create the common header file with all required definitions"""
        h_path = os.path.join(self.include_dir, 'sod_common.h')
        
        header_content = """
/* 
 * sod_common.h - Common definitions for the SOD library
 * Generated from the original monolithic code
 */

#ifndef SOD_COMMON_H__
#define SOD_COMMON_H__

/* Standard includes */
"""
        # Add standard headers
        for header in STANDARD_HEADERS:
            header_content += f"#include {header}\n"
            
        # Add API export macro
        header_content += f"\n{SOD_API_EXPORT_MACRO}\n"
        
        # Add required typedefs
        header_content += f"\n{REQUIRED_TYPEDEFS}\n"
        
        # Add required constants
        header_content += "\n/* Required constants */\n"
        for constant in REQUIRED_CONSTANTS:
            # Find the macro definition
            for macro in self.macros:
                if macro.name == constant:
                    header_content += macro.content + "\n"
                    break
        
        # Add forward declarations for common types
        header_content += "\n/* Forward declarations for common types */\n"
        for type_name in COMMON_TYPES:
            header_content += f"typedef struct {type_name} {type_name};\n"
        
        # Add common enums
        header_content += "\n/* Common enumerations */\n"
        for enum in self.enums:
            if enum.name in COMMON_ENUMS:
                header_content += enum.content + "\n\n"
        
        header_content += "\n#endif /* SOD_COMMON_H__ */\n"
        
        # Write the file
        with open(h_path, 'w', encoding='utf-8') as f:
            f.write(header_content)
            
        print(f"Created common header file")

def main():
    parser = argparse.ArgumentParser(description='Split SOD monolithic C file into components')
    parser.add_argument('--input', required=True, help='Path to the monolithic SOD.c file')
    parser.add_argument('--output-dir', required=True, help='Path to output directory')
    parser.add_argument('--verbose', action='store_true', help='Enable verbose output')
    parser.add_argument('--skip-verification', action='store_true', help='Skip verification step')
    parser.add_argument('--max-time', type=int, default=300, help='Maximum time in seconds for the entire process')
    parser.add_argument('--fix-issues', action='store_true', help='Automatically fix common issues in output files')
    parser.add_argument('--strict', action='store_true', help='Fail on any warnings or errors')
    args = parser.parse_args()
    
    # Set a global timeout for the entire process
    start_time = time.time()
    max_time = args.max_time
    
    try:
        splitter = EnhancedSodSplitter(args.input, args.output_dir)
        
        # Extract and process with timeout monitoring
        print("Starting SOD library splitting process...")
        splitter.extract_and_process()
        
        # Check if we should skip verification
        if args.skip_verification:
            print("Skipping verification step as requested.")
        elif time.time() - start_time > max_time * 0.8:  # If we've used 80% of our time budget
            print("Warning: Approaching timeout limit. Skipping verification step.")
        else:
            # Verify the output
            print("\nVerifying output files...")
            issues_found = splitter._verify_output()
            
            # If strict mode is enabled and issues were found, exit with error
            if args.strict and issues_found:
                print("Error: Issues were found during verification and strict mode is enabled.")
                sys.exit(1)
        
        print("Processing completed successfully.")
        
        # Print usage instructions
        print("\nUsage instructions:")
        print("  1. The split files are located in:")
        print(f"     - Source files: {splitter.src_dir}")
        print(f"     - Header files: {splitter.include_dir}")
        print("  2. To compile the split files, include them in your build system.")
        print("  3. To use the SOD library, include the main header: #include \"sod/sod.h\"")
    except KeyboardInterrupt:
        print("\nProcess interrupted by user. Partial results may have been saved.")
    except Exception as e:
        print(f"Error during processing: {str(e)}")
        import traceback
        traceback.print_exc()
        sys.exit(1)

if __name__ == "__main__":
    # Import time module for timeout handling
    import time
    main()
