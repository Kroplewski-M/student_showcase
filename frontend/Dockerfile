# ===== DEV =====
FROM node:22-alpine AS dev
WORKDIR /app
COPY . .
EXPOSE 3000
CMD ["sh", "-c", "npm install && npm run dev"]

# ===== BUILD =====
FROM node:20-alpine AS build
WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .
RUN npm run build

# ===== PROD =====
FROM node:20-alpine AS prod
WORKDIR /app
ENV NODE_ENV=production
COPY --from=build /app ./
EXPOSE 3000
CMD ["npm", "start"]

