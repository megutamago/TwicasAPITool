import { SupportingList } from '../types/SupportingList';
import { sortTable, updateSortIcons } from '../utils/sort';

let sortOrder: ('asc' | 'desc')[] = [];

export async function diplayTable(displayMsgEl: HTMLElement | null, props: SupportingList) {
  const supportingData = props[1];

  if (displayMsgEl) {
    displayMsgEl.innerHTML = `
      <table id="supportingListTable">
        <thead>
          <tr>
            <th>ユーザーID</th>
            <th>ユーザー名</th>
            <th id="sortButton2">point <span id="sortIcon2" class="sort-icon">&nbsp;&nbsp;&nbsp;</span></th>
            <th id="sortButton3">total_point <span id="sortIcon3" class="sort-icon">&nbsp;&nbsp;&nbsp;</span></th>
          </tr>
        </thead>
        <tbody>
          ${supportingData.map((item) => `
            <tr>
              <td>${item.screen_id}</td>
              <td>${item.name}</td>
              <td>${item.point}</td>
              <td>${item.total_point}</td>
            </tr>
          `).join('')}
        </tbody>
      </table>
    `;
  }

  // ページロード時に初期のソート順を設定する
  sortOrder = ['asc', 'asc'];
  updateSortIcons(sortOrder);

  // 昇順、降順ボタンのクリックイベントを設定
  const sortButton2 = document.getElementById('sortButton2');
  const sortButton3 = document.getElementById('sortButton3');

  if (sortButton2 && sortButton3) {
    sortButton2.addEventListener('click', () => sortHandler(2));
    sortButton3.addEventListener('click', () => sortHandler(3));
  }
}

function sortHandler(colIndex: number) {
  sortTable(colIndex, sortOrder);
}