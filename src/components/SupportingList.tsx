import { SupportingList } from '../types/SupportingList';
import { sortTable, updateSortIcons } from '../utils/sort';

let sortOrder: ('asc' | 'desc')[] = [];

export async function diplayTable(displayMsgEl: HTMLElement | null, props: SupportingList) {
  const total = props[0];
  const supportingData = props[1];

  if (displayMsgEl) {
    displayMsgEl.innerHTML = `
      <h3>サポートしている人の合計：${total}</h3>
      <table id="supportingListTable">
        <thead>
          <tr>
            <th>番号</th>
            <th>ユーザーID</th>
            <th>ユーザー名</th>
            <th id="sortButton3">アイテム・スコア <span id="sortIcon3" class="sort-icon">&nbsp;&nbsp;&nbsp;</span></th>
            <th id="sortButton4">累計スコア <span id="sortIcon4" class="sort-icon">&nbsp;&nbsp;&nbsp;</span></th>
            <th>サポートした日時</th>
          </tr>
        </thead>
        <tbody>
          ${supportingData.map((item) => `
            <tr>
              <td>${item._id}</td>
              <td>${item.screen_id}</td>
              <td>${item.name}</td>
              <td>${item.point}</td>
              <td>${item.total_point}</td>
              <td>${new Date(item.supported * 1000).toLocaleString()}</td>
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
  const sortButton3 = document.getElementById('sortButton3');
  const sortButton4 = document.getElementById('sortButton4');

  if (sortButton3 && sortButton4) {
    sortButton3.addEventListener('click', () => sortHandler(3));
    sortButton4.addEventListener('click', () => sortHandler(4));
  }
}

function sortHandler(colIndex: number) {
  sortTable(colIndex, sortOrder);
}