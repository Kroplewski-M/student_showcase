async function fakeSearchStudents(query: string) {
  await new Promise((resolve) => setTimeout(resolve, 3000));
  return [] as { id: number; name: string }[];
}

interface StudentsResultProps {
  query: string;
}

export default async function StudentsResult({ query }: StudentsResultProps) {
  const students = await fakeSearchStudents(query);

  return <div className=""></div>;
}
