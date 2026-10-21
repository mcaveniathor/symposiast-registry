// Code remains selectable and navigation works when scripts are unavailable.
const status = document.createElement('span');
status.className = 'visually-hidden';
status.setAttribute('role', 'status');
document.body.append(status);

for (const block of document.querySelectorAll('pre')) {
  const code = block.querySelector('code');
  if (!code) continue;
  const frame = document.createElement('div');
  frame.className = 'code-frame';
  block.before(frame);
  frame.append(block);
  const button = document.createElement('button');
  button.type = 'button';
  button.className = 'copy-code';
  button.textContent = 'Copy';
  button.setAttribute('aria-label', 'Copy code');
  frame.append(button);
  button.addEventListener('click', async () => {
    try {
      await navigator.clipboard.writeText(code.textContent);
      button.textContent = 'Copied';
      status.textContent = 'Code copied to clipboard.';
    } catch {
      const range = document.createRange();
      range.selectNodeContents(code);
      const selection = window.getSelection();
      selection.removeAllRanges();
      selection.addRange(range);
      status.textContent = 'Code selected. Use your browser’s copy command.';
    }
    window.setTimeout(() => { button.textContent = 'Copy'; }, 1800);
  });
}

const search = document.querySelector('#search');
if (search) {
  const entries = [...document.querySelectorAll('.catalog-entry')];
  const count = document.querySelector('#result-count');
  document.querySelector('#search-controls').hidden = false;
  const filter = () => {
    const terms = search.value.trim().toLowerCase().split(/\s+/).filter(Boolean);
    let visible = 0;
    for (const entry of entries) {
      entry.hidden = !terms.every(term => entry.dataset.search.includes(term));
      if (!entry.hidden) visible++;
    }
    count.textContent = `${visible} of ${entries.length} releases shown`;
    document.querySelector('#empty-state').hidden = visible !== 0;
    const url = new URL(location.href);
    if (search.value.trim()) url.searchParams.set('q', search.value.trim());
    else url.searchParams.delete('q');
    history.replaceState(null, '', url);
  };
  search.value = new URL(location.href).searchParams.get('q') || '';
  search.addEventListener('input', filter);
  filter();
}
