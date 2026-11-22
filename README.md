<div align="center">

<img src="banner-rounded.png" alt="finteredit banner" width="100%">

# ⛏️ finteredit

**Двухточечное выделение блоков, застройка области и отмена для Horizon**

<img src="https://img.shields.io/badge/version-0.1.4-7c5cff?style=for-the-badge" alt="version">
<img src="https://img.shields.io/badge/license-MIT-00c853?style=for-the-badge" alt="license">
<img src="https://img.shields.io/badge/Horizon-26.3-ff7a00?style=for-the-badge" alt="game">
<img src="https://img.shields.io/badge/built%20with-Rust-dea584?style=for-the-badge" alt="rust">

---

### ✨ Что это

Небольшой мод для **Horizon** (Minecraft: Java Edition 26.3, переписанный на Rust).
Он даёт «волшебный топор»: ЛКМ ставит первую точку, ПКМ — вторую,
после чего выделенную область можно застроить и отменить действие.

---

### 🎮 Команды

<table align="center" cellpadding="8" cellspacing="0">
  <tr><th>Команда</th><th>Действие</th></tr>
  <tr><td><code>/wand</code> или <code>//wand</code></td><td>Выдать деревянный топор и сбросить выделение</td></tr>
  <tr><td><b>ЛКМ</b> по блоку с топором</td><td>Поставить первую точку. Блок не ломается</td></tr>
  <tr><td><b>ПКМ</b> по блоку с топором</td><td>Поставить вторую точку. Блок не используется</td></tr>
  <tr><td><code>//set &lt;n&gt;</code> <b>[оп 2]</b></td><td>Застроить выделенную область блоком под номером <code>n</code></td></tr>
  <tr><td><code>//undo</code> или <code>/undo</code> <b>[оп 2]</b></td><td>Отменить последнюю застройку</td></tr>
</table>

---

### 🧱 Доступные блоки

<table align="center" cellpadding="6" cellspacing="0">
  <tr><th><code>n</code></th><th>Блок</th><th><code>n</code></th><th>Блок</th></tr>
  <tr><td><code>0</code></td><td>air</td><td><code>5:1</code></td><td>oak_planks</td></tr>
  <tr><td><code>1</code></td><td>stone</td><td><code>5:2</code></td><td>spruce_planks</td></tr>
  <tr><td><code>2</code></td><td>grass_block</td><td><code>5:3</code></td><td>birch_planks</td></tr>
  <tr><td><code>3</code></td><td>dirt</td><td><code>5:4</code></td><td>jungle_planks</td></tr>
  <tr><td><code>4</code></td><td>cobblestone</td><td><code>5:5</code></td><td>acacia_planks</td></tr>
  <tr><td><code>6</code></td><td>bedrock</td><td></td><td></td></tr>
</table>

Пример: <code>//set 5:3</code> — берёзовые доски.

---

### 🔒 Ограничения

- <code>//set</code>, <code>//undo</code> и <code>/undo</code> требуют уровень оператора <b>2</b>.
- Максимальный размер выделения: <b>32768</b> блоков.
- История отмены хранит до <b>8</b> действий и очищается при выходе игрока.
- Выделение и отмена привязаны к измерению.
- Отмена восстанавливает только блоки; содержимое сундуков не восстанавливается.

---

### 🌍 Локализация

Сообщения переведены на русский и английский и лежат в <code>assets/finteredit/lang/</code>.
Язык выбирается по языку клиента. Как добавить язык — см. <code>LANGUAGES.md</code>.

---

### 📦 Установка

1. Скопируйте <code>finteredit.hmod</code> в папку инстанса <code>horizon-mods/</code>.
2. Добавьте строку <code>mod.finteredit:true</code> в <code>horizon-mods.txt</code>.
3. Полностью перезапустите игру.

Готовый пакет лежит в <code>releases/</code>.

---

### 🛠️ Сборка

<pre><code>cargo horizon-mod build --wasm</code></pre>

---

### 📁 Структура

<pre><code>
src/
├── lib.rs
├── blocks.rs
├── commands.rs
├── editor.rs
├── i18n.rs
├── interact.rs
└── selection.rs

assets/finteredit/lang/
├── en_us.json
└── ru_ru.json
</code></pre>

---

🌐 <b>Сайт:</b> ссылка будет добавлена

👤 Автор: <b>sinvides</b> &nbsp;·&nbsp; 📄 Лицензия: <b>MIT</b>

<small>Horizon Nightly 2026-11-22 (2b77287) · игра 26.3 · версия 0.1.4</small>

</div>
