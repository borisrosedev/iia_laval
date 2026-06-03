import bcrypt from "bcryptjs";
import cors from "cors";
import express from "express";
import rateLimit from "express-rate-limit";
import helmet from "helmet";
import morgan from "morgan";
import { requireAuth, signToken } from "./auth.js";
import { prisma } from "./prisma.js";
import { loginSchema, registerSchema, userSchema } from "./validators.js";

const app = express();
const corsOrigin = process.env.CORS_ORIGIN || "http://localhost:3000";

app.set("trust proxy", 1);
app.use(helmet());
app.use(
  cors({
    origin: corsOrigin
  })
);
app.use(express.json());
app.use(morgan("dev"));
app.use(
  rateLimit({
    windowMs: 15 * 60 * 1000,
    max: 100,
    standardHeaders: true,
    legacyHeaders: false,
    message: { message: "Trop de requêtes, réessaie plus tard." }
  })
);

app.get("/health", (req, res) => {
  res.json({ status: "ok", instance: process.env.HOSTNAME || "backend" });
});

app.post("/auth/register", async (req, res) => {
  const parsed = registerSchema.safeParse(req.body);

  if (!parsed.success) {
    return res.status(400).json({ message: "Données invalides." });
  }

  const { email, password, firstName, lastName } = parsed.data;
  const existingUser = await prisma.user.findUnique({ where: { email } });

  if (existingUser) {
    return res.status(409).json({ message: "Cet utilisateur existe déjà." });
  }

  const hashedPassword = await bcrypt.hash(password, 10);
  const user = await prisma.user.create({
    data: {
      email,
      password: hashedPassword,
      firstName,
      lastName
    }
  });

  const token = signToken(user);

  res.status(201).json({
    message: "Inscription réussie.",
    token,
    user: sanitizeUser(user)
  });
});

app.post("/auth/login", async (req, res) => {
  const parsed = loginSchema.safeParse(req.body);

  if (!parsed.success) {
    return res.status(400).json({ message: "Données invalides." });
  }

  const { email, password } = parsed.data;
  const user = await prisma.user.findUnique({ where: { email } });

  if (!user) {
    return res.status(401).json({ message: "Identifiants invalides." });
  }

  const isValid = await bcrypt.compare(password, user.password);

  if (!isValid) {
    return res.status(401).json({ message: "Identifiants invalides." });
  }

  const token = signToken(user);

  res.json({
    message: "Connexion réussie.",
    token,
    user: sanitizeUser(user)
  });
});

app.get("/users", requireAuth, async (req, res) => {
  const users = await prisma.user.findMany({
    orderBy: { id: "asc" }
  });

  res.json(users.map(sanitizeUser));
});

app.get("/users/:id", requireAuth, async (req, res) => {
  const id = Number(req.params.id);

  if (Number.isNaN(id)) {
    return res.status(400).json({ message: "Identifiant invalide." });
  }

  const user = await prisma.user.findUnique({ where: { id } });

  if (!user) {
    return res.status(404).json({ message: "Utilisateur introuvable." });
  }

  res.json(sanitizeUser(user));
});

app.post("/users", requireAuth, async (req, res) => {
  const parsed = userSchema.safeParse(req.body);

  if (!parsed.success || !parsed.data.password) {
    return res.status(400).json({ message: "Données invalides." });
  }

  const { email, password, firstName, lastName, role } = parsed.data;
  const existingUser = await prisma.user.findUnique({ where: { email } });

  if (existingUser) {
    return res.status(409).json({ message: "Cet utilisateur existe déjà." });
  }

  const hashedPassword = await bcrypt.hash(password, 10);
  const user = await prisma.user.create({
    data: {
      email,
      password: hashedPassword,
      firstName,
      lastName,
      role
    }
  });

  res.status(201).json(sanitizeUser(user));
});

app.put("/users/:id", requireAuth, async (req, res) => {
  const id = Number(req.params.id);
  const parsed = userSchema.partial().safeParse(req.body);

  if (Number.isNaN(id)) {
    return res.status(400).json({ message: "Identifiant invalide." });
  }

  if (!parsed.success) {
    return res.status(400).json({ message: "Données invalides." });
  }

  const user = await prisma.user.findUnique({ where: { id } });

  if (!user) {
    return res.status(404).json({ message: "Utilisateur introuvable." });
  }

  const data = { ...parsed.data };

  if (data.password) {
    data.password = await bcrypt.hash(data.password, 10);
  }

  try {
    const updatedUser = await prisma.user.update({
      where: { id },
      data
    });

    return res.json(sanitizeUser(updatedUser));
  } catch {
    return res.status(409).json({ message: "Email déjà utilisé." });
  }
});

app.delete("/users/:id", requireAuth, async (req, res) => {
  const id = Number(req.params.id);

  if (Number.isNaN(id)) {
    return res.status(400).json({ message: "Identifiant invalide." });
  }

  const user = await prisma.user.findUnique({ where: { id } });

  if (!user) {
    return res.status(404).json({ message: "Utilisateur introuvable." });
  }

  await prisma.user.delete({ where: { id } });

  res.status(204).send();
});

app.use((error, req, res, next) => {
  console.error(error);
  res.status(500).json({ message: "Erreur interne du serveur." });
});

function sanitizeUser(user) {
  const { password, ...safeUser } = user;
  return safeUser;
}

export default app;
