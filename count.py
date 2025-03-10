#!/usr/bin/env python3
import os
import sys
from collections import defaultdict

def is_text_file(file_path):
    """Check if a file is a text file based on its extension."""
    # Add or remove extensions based on your project's file types
    text_extensions = {
        '.txt', '.md', '.js', '.jsx', '.ts', '.tsx', '.html', '.css', '.scss',
        '.sass', '.less', '.json', '.xml', '.yaml', '.yml', '.py', '.rb', '.php',
        '.java', '.c', '.cpp', '.h', '.cs', '.go', '.rs', '.swift', '.kt', '.sh'
    }
    _, ext = os.path.splitext(file_path.lower())
    return ext in text_extensions

def count_lines_in_file(file_path):
    """Count the number of lines in a file."""
    try:
        with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
            return len(f.readlines())
    except Exception as e:
        print(f"Error reading file {file_path}: {e}", file=sys.stderr)
        return 0

def walk_directory(directory):
    """Walk through directory recursively and yield file paths."""
    for root, _, files in os.walk(directory):
        for file in files:
            yield os.path.join(root, file)

def count_all_lines(project_dir):
    """Count lines in all text files in the project directory."""
    total_lines = 0
    file_count = 0
    file_stats = defaultdict(lambda: {'files': 0, 'lines': 0})
    
    for file_path in walk_directory(project_dir):
        if is_text_file(file_path):
            lines = count_lines_in_file(file_path)
            total_lines += lines
            file_count += 1
            
            # Get extension for statistics
            _, ext = os.path.splitext(file_path.lower())
            file_stats[ext]['files'] += 1
            file_stats[ext]['lines'] += lines
            
            # Optional: log each file (comment out for large projects)
            print(f"{file_path}: {lines} lines")
    
    print('\n--- Summary ---')
    print(f"Total text files: {file_count}")
    print(f"Total lines: {total_lines}")
    
    print('\n--- By File Type ---')
    # Sort by line count (descending)
    for ext, stats in sorted(file_stats.items(), key=lambda x: x[1]['lines'], reverse=True):
        print(f"{ext}: {stats['files']} files, {stats['lines']} lines")

if __name__ == "__main__":
    project_dir = sys.argv[1] if len(sys.argv) > 1 else '.'
    print(f"Counting lines in: {os.path.abspath(project_dir)}")
    count_all_lines(project_dir)
