import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import Award from "../SVGS/Award";
import { UserProfile } from "./page";
import { getLinkIcon } from "../components/LinkIcon";
import { isSafeLink } from "../lib/helpers";

interface Props {
  user: UserProfile;
}

export default function ProfileInfo({ user }: Props) {
  return (
    <>
      <div className="min-w-[200px] flex-1">
        <div className="mb-1 flex flex-wrap items-start justify-between gap-3">
          <div>
            {user.firstName || user.lastName ? (
              <>
                <h2 className="mb-0.5 text-2xl font-bold text-white">
                  {user.firstName} {user.lastName}
                </h2>
              </>
            ) : (
              <>Name not set</>
            )}

            {user.courseName && (
              <p className="text-sm text-secondary/50">{user.courseName}</p>
            )}
          </div>
        </div>
        <div>
          <p>{user.personalEmail}</p>
        </div>
        {/* description */}
        {user.description ? (
          <div className="mt-4 rounded-lg border border-secondary/10 bg-secondary/5 px-4 py-3">
            <p className="text-sm leading-relaxed text-secondary/70">
              {user.description}
            </p>
          </div>
        ) : (
          <p className="mt-4 text-sm italic text-secondary/40">
            No description added
          </p>
        )}
        <div className="flex flex-wrap items-start md:gap-16 gap-2">
          {/* tech interests */}
          <div className="mt-6 ">
            <p className="text-xs font-semibold uppercase tracking-wider text-secondary/50">
              Tech Interests ({user.tools?.length ?? 0})
            </p>
            {user.tools != undefined && user.tools.length > 0 ? (
              <div className="mt-3 flex flex-wrap gap-2">
                {user.tools.map((tool, key) => (
                  <span
                    className="inline-flex items-center rounded-full border border-secondary/15 bg-secondary/5 px-3 py-1 text-xs font-medium text-secondary/80 transition-colors hover:border-secondary/30 hover:bg-secondary/10"
                    key={key}
                  >
                    {tool}
                  </span>
                ))}
              </div>
            ) : (
              <p className="mt-3 text-sm italic text-secondary/40">
                No tools added
              </p>
            )}
          </div>
          {/* certificates */}
          <div className="mt-6 block">
            <p className="text-xs font-semibold uppercase tracking-wider text-secondary/50">
              Certificates ({user.certificates?.length ?? 0})
            </p>
            {user.certificates != undefined && user.certificates.length > 0 ? (
              <div className="mt-3 flex flex-wrap gap-2">
                {user.certificates.map((certificate, key) => (
                  <span
                    className="inline-flex items-center gap-1.5 rounded-lg border border-secondary/15 bg-secondary/5 px-3 py-1 text-xs font-medium text-secondary/80 transition-colors hover:border-secondary/30 hover:bg-secondary/10"
                    key={key}
                  >
                    <Award />
                    {certificate}
                  </span>
                ))}
              </div>
            ) : (
              <p className="mt-3 text-sm italic text-secondary/40">
                No certificates added
              </p>
            )}
          </div>
        </div>
        {/* Links */}
        <div className="mt-8">
          <p className="text-xs font-semibold uppercase tracking-wider text-secondary/50">
            Links ({user.links?.length ?? 0})
          </p>
          {user.links != undefined && user.links.length > 0 ? (
            <div className="mt-3 flex flex-wrap gap-3">
              {user.links
                .filter((x) => isSafeLink(x.url))
                .map((link, key) => (
                  <a
                    key={key}
                    href={link.url}
                    target="_blank"
                    rel="noopener noreferrer"
                    title={link.linkType}
                    aria-label={`Open ${link.linkType} link`}
                    className="inline-flex h-10 px-5  items-center justify-center rounded-lg border border-secondary/15 bg-secondary/5 text-secondary/60 transition-all hover:border-secondary/30 hover:bg-secondary/10 hover:text-secondary"
                  >
                    <FontAwesomeIcon icon={getLinkIcon(link.linkType)} />
                    <p className="pl-1">{link.name ?? link.linkType}</p>
                  </a>
                ))}
            </div>
          ) : (
            <p className="mt-3 text-sm italic text-secondary/40">
              No links added
            </p>
          )}
        </div>
      </div>
    </>
  );
}
