// linkIcon.tsx
import { IconDefinition } from "@fortawesome/fontawesome-svg-core";
import {
  faGithub,
  faYoutube,
  faLinkedin,
  faGitlab,
  faBitbucket,
  faStackOverflow,
  faFigma,
} from "@fortawesome/free-brands-svg-icons";
import {
  faArrowUpRightFromSquare,
  faLink,
} from "@fortawesome/free-solid-svg-icons";

export function getLinkIcon(linkType: string): IconDefinition {
  switch (linkType.toLowerCase()) {
    case "github":
      return faGithub;
    case "youtube":
      return faYoutube;
    case "linkedin":
      return faLinkedin;
    case "gitlab":
      return faGitlab;
    case "bitbucket":
      return faBitbucket;
    case "stack overflow":
      return faStackOverflow;
    case "figma":
      return faFigma;
    case "live preview":
      return faArrowUpRightFromSquare;
    default:
      return faLink;
  }
}
