import { faGithub } from "@fortawesome/free-brands-svg-icons/faGithub";
import { faLinkedin } from "@fortawesome/free-brands-svg-icons/faLinkedin";
import { faGlobe } from "@fortawesome/free-solid-svg-icons/faGlobe";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

export default function Footer() {
  return (
    <>
      <div className="w-full bg-support z-500 py-1">
        <p className="text-center text-dark font-bold">
          Website made by{" "}
          <a
            href="https://github.com/Kroplewski-M/student_showcase"
            target="_blank"
            className="underline"
          >
            Mateusz Kroplewski
          </a>
        </p>
        <div className="flex justify-center">
          <a href="https://github.com/Kroplewski-M" target="_blank">
            <FontAwesomeIcon icon={faGithub} size="xl" className="text-dark" />
          </a>
          <a href="https://www.mateusz-k.dev/" target="_blank">
            <FontAwesomeIcon icon={faGlobe} size="xl" className="text-dark" />
          </a>
          <a
            href="https://www.linkedin.com/in/mateusz-kroplewski-732239176/"
            target="_blank"
          >
            <FontAwesomeIcon
              icon={faLinkedin}
              size="xl"
              className="text-dark"
            />
          </a>
        </div>
      </div>
    </>
  );
}
