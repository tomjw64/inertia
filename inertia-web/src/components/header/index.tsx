import { Link } from 'preact-router/match';
import style from './style.module.scss';

const Header = () => (
  <header class={style.header}>
    <a href="/" class={style.logo}>
      <img
        src="preact-logo-inverse.svg"
        alt="Preact Logo"
        height="32"
        width="32"
      />
      <h1>Preact CLI</h1>
    </a>
    <nav>
      <Link activeClassName={style.active} href="/">
        Home
      </Link>
    </nav>
  </header>
);

export default Header;
