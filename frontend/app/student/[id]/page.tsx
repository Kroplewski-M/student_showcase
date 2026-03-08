import { UserProfile } from "@/app/profile/page";
import ProfileView from "@/app/profile/ProfileView";
import ErrorSVG from "@/app/SVGS/ErrorSVG";
import Link from "next/link";
import { redirect } from "next/navigation";

export default async function Student({
  params,
}: {
  params: Promise<{ id: string }>;
}) {
  const studentId = (await params).id;
  let profile: UserProfile | null = null;
  let error: string | null = null;
  let notFound = false;
  try {
    const res = await fetch(
      `${process.env.API_INTERNAL_URL}/user/info/${studentId}`,
      {
        cache: "no-store",
      },
    );
    if (!res.ok) {
      notFound = true;
    } else {
      profile = await res.json();
    }
  } catch {
    error = "Unable to connect to the server. Please try again later.";
  }
  if (notFound) {
    redirect("/404");
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
            href="/"
            className="mt-1 rounded-xl border border-third/40 bg-third/10 px-8 py-3 text-sm font-semibold text-light backdrop-blur-sm transition-all hover:border-third/60 hover:bg-third/20 active:scale-[0.985] cursor-pointer"
          >
            Back Home
          </Link>
        </div>
      </div>
    );
  }

  return <ProfileView profile={profile} />;
}
