import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faBan } from "@fortawesome/free-solid-svg-icons";
import Link from "next/link";

export default function AccountSuspended() {
  return (
    <div className="flex flex-col items-center justify-center gap-6 min-h-screen px-4">
      <div className="flex flex-col items-center gap-5 rounded-2xl border border-danger/20 bg-danger/5 px-10 py-10 backdrop-blur-sm max-w-md w-full text-center">
        <div className="flex h-14 w-14 items-center justify-center rounded-full bg-danger/10 border border-danger/20">
          <FontAwesomeIcon icon={faBan} className="w-7 h-7 text-danger" />
        </div>
        <div className="flex flex-col gap-2">
          <h2 className="text-lg font-semibold text-light">
            Account Suspended
          </h2>
          <p className="text-sm text-light/60 leading-relaxed">
            Your account has been suspended. If you believe this is a mistake,
            please contact your administrator.
          </p>
        </div>
        <Link
          href="/"
          className="mt-1 rounded-xl border border-third/40 bg-third/10 px-8 py-3 text-sm font-semibold text-light backdrop-blur-sm transition-all hover:border-third/60 hover:bg-third/20 active:scale-[0.985] cursor-pointer"
        >
          Return Home
        </Link>
      </div>
    </div>
  );
}
