import os

docs_dir = 'docs'
index_file = os.path.join(docs_dir, 'index.md')

markdown_files = [f for f in os.listdir(docs_dir) if f.endswith('.md') and f != 'index.md']
markdown_files.sort()

with open(index_file, 'w') as f:
    f.write('# Documentation Index\n\n')
    for filename in markdown_files:
        title = filename.replace('.md', '').replace('-', ' ').title()
        f.write(f'- [{title}]({filename})\n')

print(f"Index generated at {index_file}")
