ARG PUBLIC_BACKEND_URL
ARG PORT

FROM node:latest as builder
WORKDIR /app
COPY . .
RUN npm i vite
RUN npm run build


FROM node:slim
WORKDIR /app

COPY --from=builder /app/package.json .
COPY --from=builder /app/build ./build
COPY --from=builder /app/node_modules ./node_modules

ENV PORT 6789
ENV PUBLIC_BACKEND_URL=$PUBLIC_BACKEND_URL

EXPOSE $PORT
CMD node -r dotenv/config build
