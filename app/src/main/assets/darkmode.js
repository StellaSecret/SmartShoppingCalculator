/* DARK MODE */
function toggleDark() {
  const isDark = document.body.classList.toggle('dark');
  localStorage.setItem('theme', isDark ? 'dark' : 'light');
  document.getElementById('dark-btn').textContent = isDark ? '☀️ light' : '🌙 dark';
}
if (localStorage.getItem('theme') === 'dark' ||
    (!localStorage.getItem('theme') && matchMedia('(prefers-color-scheme: dark)').matches)) {
  document.body.classList.add('dark');
  document.getElementById('dark-btn').textContent = '☀️ light';
}

