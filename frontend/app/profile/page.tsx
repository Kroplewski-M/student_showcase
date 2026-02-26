import { redirect } from "next/navigation";
import { getUser } from "../lib/auth";
import ProfileView from "./ProfileView";
import Link from "next/link";
import ErrorSVG from "../SVGS/ErrorSVG";

export interface Links {
  linkType: string;
  linkUrl: string;
}
export interface UserProfile {
  id: string;
  profileImageName: string | null;
  firstName: string | null;
  lastname: string | null;
  personalEmail: string | null;
  courseName: string | null;
  certificates: string[] | null;
  links: Links[] | null;
}

export default async function ProfilePage() {
  const user = await getUser();
  if (!user) redirect("/login");

  let profile: UserProfile | null = null;
  let error: string | null = null;

  try {
    const res = await fetch(
      `${process.env.API_INTERNAL_URL}/user/info/${user.id}`,
      {
        cache: "no-store",
      },
    );
    if (!res.ok) {
      error =
        res.status === 404
          ? "Profile not found."
          : "Something went wrong loading your profile.";
    } else {
      profile = await res.json();
      console.log(profile);
    }
  } catch {
    error = "Unable to connect to the server. Please try again later.";
  }

  if (error || !profile) {
    return (
      <div className="flex flex-col items-center justify-center gap-6 min-h-screen px-4">
        <div className="flex flex-col items-center gap-5 rounded-2xl border border-red-500/20 bg-red-500/5 px-10 py-10 backdrop-blur-sm max-w-md w-full text-center">
          <div className="flex h-14 w-14 items-center justify-center rounded-full bg-red-500/10 border border-red-500/20">
            <ErrorSVG />
          </div>
          <div className="flex flex-col gap-2">
            <h2 className="text-lg font-semibold text-light">
              Something went wrong
            </h2>
            <p className="text-sm text-light/60 leading-relaxed">
              {error ?? "An unexpected error occurred."}
            </p>
          </div>

          <Link
            href="/profile"
            className="mt-1 rounded-xl border border-third/40 bg-third/10 px-8 py-3 text-sm font-semibold text-light backdrop-blur-sm transition-all hover:border-third/60 hover:bg-third/20 active:scale-[0.985] cursor-pointer"
          >
            Try again
          </Link>
        </div>
      </div>
    );
  }

  return <ProfileView profile={profile} />;
}
