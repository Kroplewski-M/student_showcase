import { redirect } from "next/navigation";
import { getUser } from "../lib/auth";
import ProfileView from "./ProfileView";

export interface UserProfile {
  id: string;
  profile_image_name: string | null;
}

export default async function ProfilePage() {
  const user = await getUser();
  if (!user) redirect("/login");

  const res = await fetch(
    `${process.env.API_INTERNAL_URL}/user/info/${user.id}`,
    { cache: "no-store" },
  );

  if (!res.ok) redirect("/");

  const profile: UserProfile = await res.json();

  return <ProfileView profile={profile} />;
}
