export interface siteInfoDto {
  studentCount: number | null;
  projectCount: number | null;
}
export interface AuthenticatedUser {
  id: string;
  is_admin: boolean;
}
