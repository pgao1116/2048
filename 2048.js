const startGame = document.querySelector('start');
const gameContainer = document.querySelector('.game-container');
const score = document.querySelector('.score');
const bestScore = document.querySelector('.best-score');
const board = document.querySelector('.board');
const gameOver = document.querySelector('.game-over');
const gameOverScore = document.querySelector('.game-over-score');
const gameOverBestScore = document.querySelector('.game-over-best-score');
const restart = document.querySelector('.restart');
let gameData = [];
let currentScore = 0;
let best = 0;
let isGameOver = false;
    
startGame.addEventListener('click', (e) => {
  e.preventDefault;
  startGame.style.display = 'none';
  gameContainer.style.display = 'flex';
  init();
})

init = () => {
  gameData = Array.from({ length: 4 }, () => Array.from({ length: 4 }, () => 0));
  currentScore = 0;
  isGameOver = false;
  best = localStorage.getItem('best') ? localStorage.getItem('best') : 0;
  bestScore.innerHTML = best;
  gameOver.style.display = 'none';
  gameOverScore.innerHTML = 0;
  gameOverBestScore.innerHTML = 0;
  render();
  addNumber();
  addNumber();
}

render = () => {
  board.innerHTML = '';
  gameData.forEach((row, i) => {
    row.forEach((cell, j) => {
      if (cell > 0) {
        const cellElem = document.createElement('div');
        cellElem.classList.add(`cell-${i}-${j}`, 'cell');
        cellElem.innerHTML = cell;
        board.appendChild(cellElem);
      }
    })
  })
  score.innerHTML = currentScore;
}

addNumber = () => {
  const emptyCells = [];
  gameData.forEach((row, i) => {
    row.forEach((cell, j) => {
      if (cell === 0) {
        emptyCells.push({ i, j });
      }
    })
  })
  if (emptyCells.length === 0) {
    return;
  }
  const { i, j } = emptyCells[Math.floor(Math.random() * emptyCells.length)];
  gameData[i][j] = Math.random() > 0.5 ? 2 : 4;
  render();
}

