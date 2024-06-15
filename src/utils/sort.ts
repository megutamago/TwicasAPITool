export function sortTable(colIndex: number, sortOrder: ('asc' | 'desc')[]) {
  if (colIndex !== 2 && colIndex !== 3) {
    return;
  }

  const table = document.getElementById("supportingListTable");
  if (!table) return;

  const tbody = table.querySelector('tbody');
  if (!tbody) return;

  const rows = Array.from(tbody.rows);

  // Toggle sort order (asc/desc)
  sortOrder[colIndex - 2] = sortOrder[colIndex - 2] === 'asc' ? 'desc' : 'asc';

  // Update sort icons
  updateSortIcons(sortOrder);

  rows.sort((rowA, rowB) => {
    let cellA = parseFloat(rowA.cells[colIndex].textContent!.trim());
    let cellB = parseFloat(rowB.cells[colIndex].textContent!.trim());

    if (sortOrder[colIndex - 2] === 'asc') {
      return cellA - cellB;
    } else {
      return cellB - cellA;
    }
  });

  // Clear and re-append sorted rows
  tbody.innerHTML = '';
  rows.forEach(row => tbody.appendChild(row));
}

export function updateSortIcons(sortOrder: ('asc' | 'desc')[]) {
  const table = document.getElementById("supportingListTable");
  if (!table) return;

  const headers = table.querySelectorAll('th');

  headers.forEach((header, index) => {
    const icon = header.querySelector('.sort-icon');
    if (icon) {
      if (index === 2 || index === 3) {
        const colIndex = index - 2;
        if (sortOrder[colIndex] === 'asc') {
          icon.textContent = '▲';
        } else if (sortOrder[colIndex] === 'desc') {
          icon.textContent = '▼';
        } else {
          icon.textContent = ' ';
        }
      } else {
        icon.textContent = ' ';
      }
    }
  });
}
