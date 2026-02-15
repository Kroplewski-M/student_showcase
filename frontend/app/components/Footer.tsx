import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faGithub, faLinkedin } from "@fortawesome/free-brands-svg-icons";
import { faGlobe } from "@fortawesome/free-solid-svg-icons";
import Link from "next/link";
import LogoWritten from "./LogoWritten";

export default function Footer() {
  return (
    <footer className="relative z-10 border-t border-third/30">
      <div className="mx-auto max-w-6xl px-4 py-10 sm:px-8">
        <div className="flex flex-col items-center gap-6 sm:flex-row sm:justify-between">
          {/* Logo + credit */}
          <div className="text-center sm:text-left">
            <Link
              href="/"
              className="text-lg font-extrabold tracking-tight text-light"
            >
              <LogoWritten />
            </Link>
            <p className="mt-1 text-xs text-support/50">
              Built by{" "}
              <a
                href="https://www.mateusz-k.dev/"
                target="_blank"
                rel="noopener noreferrer"
                className="font-medium text-support/70 transition-colors hover:text-secondary"
              >
                Mateusz Kroplewski
              </a>
            </p>
          </div>

          {/* Navigation */}
          <div className="flex gap-8 text-xs font-semibold uppercase tracking-wider text-support/50">
            <Link href="/login" className="transition-colors hover:text-light">
              Login
            </Link>
            <Link
              href="/register"
              className="transition-colors hover:text-light"
            >
              Register
            </Link>
          </div>

          {/* Socials */}
          <div className="flex items-center gap-4">
            <a
              href="https://github.com/Kroplewski-M"
              target="_blank"
              rel="noopener noreferrer"
              className="text-support/40 transition-colors hover:text-secondary"
              aria-label="GitHub"
            >
              <FontAwesomeIcon icon={faGithub} size="lg" />
            </a>
            <a
              href="https://www.mateusz-k.dev/"
              target="_blank"
              rel="noopener noreferrer"
              className="text-support/40 transition-colors hover:text-secondary"
              aria-label="Portfolio"
            >
              <FontAwesomeIcon icon={faGlobe} size="lg" />
            </a>
            <a
              href="https://www.linkedin.com/in/mateusz-kroplewski-732239176/"
              target="_blank"
              rel="noopener noreferrer"
              className="text-support/40 transition-colors hover:text-secondary"
              aria-label="LinkedIn"
            >
              <FontAwesomeIcon icon={faLinkedin} size="lg" />
            </a>
          </div>
        </div>
      </div>
    </footer>
  );
}
