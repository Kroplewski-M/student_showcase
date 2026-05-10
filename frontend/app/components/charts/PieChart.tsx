import { chartData } from "@/app/lib/models";
import { ResponsiveContainer, PieChart, Pie, Tooltip, Legend } from "recharts";

interface PieChartProps {
  data: chartData[];
  width: number;
  height: number;
  title: string;
}
const withColors = (data: { name: string; value: number }[]) =>
  data.map((item, i) => ({
    ...item,
    fill: `hsl(${(i * 360) / data.length}, 65%, 55%)`,
  }));

export default function DrawPieChart({
  data,
  width,
  height,
  title,
}: PieChartProps) {
  return (
    <div className="flex flex-col items-center bg-white/5 border border-white/10 rounded-2xl p-6 shadow-lg">
      <h2 className="text-sm font-semibold text-secondary/70 mb-3">{title}</h2>
      <ResponsiveContainer width={width} height={height}>
        {data === null || data === undefined ? (
          <p>no data</p>
        ) : (
          <PieChart>
            <Pie
              data={withColors(data)}
              dataKey="value"
              cx="50%"
              cy="50%"
              outerRadius={100}
            />
            <Tooltip />
            <Legend
              iconSize={10}
              wrapperStyle={{
                whiteSpace: "nowrap",
                overflow: "scroll",
              }}
            />
          </PieChart>
        )}
      </ResponsiveContainer>
    </div>
  );
}
