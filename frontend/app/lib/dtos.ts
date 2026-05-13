export interface siteInfoDto {
  studentCount: number | null;
  projectCount: number | null;
  topInterests: string[];
}
export interface AuthenticatedUser {
  id: string;
  is_admin: boolean;
}
