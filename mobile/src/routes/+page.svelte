<div class="home">
  <h1 class="logo">EasyToGo.</h1>

  <div class="search-bar">
    <input type="text" placeholder="Search or Scan" />
    <div class="search-dot"></div>
  </div>

  <div class="quick-links">
    <button class="link">Restaurants →</button>
    <button class="link">Transit →</button>
    <a href="/medical" class="link">Medical →</a>
  </div>
</div>

<style>
  .home {
    background: white;
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    justify-content: center;
    padding: 0 2rem;
    gap: 3rem;
  }

  .logo {
    font-size: 3.5rem;
    font-weight: 900;
    letter-spacing: -0.05em;
    color: black;
  }

  .search-bar {
    width: 100%;
    position: relative;
    border-bottom: 4px solid black;
    padding-bottom: 0.5rem;
  }

  .search-bar input {
    width: 100%;
    font-size: 1.75rem;
    font-weight: 700;
    border: none;
    outline: none;
    background: transparent;
    font-family: 'Inter', sans-serif;
  }

  .search-bar input::placeholder { color: #d4d4d4; }

  .search-dot {
    position: absolute;
    right: 0;
    bottom: 0.5rem;
    width: 1.5rem;
    height: 1.5rem;
    background: black;
    border-radius: 50%;
  }

  .quick-links {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    width: 100%;
  }

  .link {
    font-size: 2rem;
    font-weight: 700;
    color: #a3a3a3;
    background: none;
    border: none;
    cursor: pointer;
    text-align: left;
    font-family: 'Inter', sans-serif;
    text-decoration: none;
    display: block;
    transition: color 0.2s;
  }

  .link:hover { color: black; }
</style>
