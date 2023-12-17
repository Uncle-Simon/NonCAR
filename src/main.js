const { invoke } = window.__TAURI__.tauri;

let alphaInputEl;
let alphaCorrectEl;
let beforeAfterEl;
let letterEl;

async function submitAnswer() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  let answers = await invoke("get_answer", {
    answer: alphaInputEl.value,
    beforeAfter: beforeAfterEl.textContent,
    letter: letterEl.textContent,
  });
  alphaCorrectEl.textContent = answers[0];
  if (answers[1]) {
    newQuestion();
  }
  alphaInputEl.focus();
  alphaInputEl.select();
}

async function newQuestion() {
  let questions = await invoke("get_new_question");
  beforeAfterEl.textContent = questions[0];
  letterEl.textContent = questions[1];
}

async function skipQuestion() {
  let answers = await invoke("skip_question", {
    beforeAfter: beforeAfterEl.textContent,
    letter: letterEl.textContent,
  });
  alphaCorrectEl.textContent = "The correct answer was: " + answers[0];
  beforeAfterEl.textContent = answers[1];
  letterEl.textContent = answers[2];
}

window.addEventListener("DOMContentLoaded", () => {
  alphaInputEl = document.querySelector("#alpha-input");
  alphaCorrectEl = document.querySelector("#alpha-correct");
  beforeAfterEl = document.querySelector("#before-after");
  letterEl = document.querySelector("#letter");
  newQuestion();
  document.querySelector("#alpha-form").addEventListener("submit", (e) => {
    e.preventDefault();
    submitAnswer();
  });
  document.querySelector("#skip").addEventListener("click", (e) => {
    skipQuestion();
  })
});