import { useEffect, useState } from "react";
import { request } from "./api";

const emptyForm = {
  email: "",
  password: "",
  firstName: "",
  lastName: "",
  role: "user"
};

export default function App() {
  const [mode, setMode] = useState("login");
  const [authForm, setAuthForm] = useState(emptyForm);
  const [userForm, setUserForm] = useState(emptyForm);
  const [users, setUsers] = useState([]);
  const [message, setMessage] = useState("");
  const [currentUser, setCurrentUser] = useState(() => {
    const savedUser = localStorage.getItem("user");
    return savedUser ? JSON.parse(savedUser) : null;
  });
  const [editingId, setEditingId] = useState(null);

  useEffect(() => {
    if (localStorage.getItem("token")) {
      loadUsers();
    }
  }, []);

  async function loadUsers() {
    try {
      const data = await request("/users");
      setUsers(data);
    } catch (error) {
      setMessage(error.message);
    }
  }

  async function handleAuthSubmit(event) {
    event.preventDefault();
    setMessage("");

    try {
      const endpoint = mode === "login" ? "/auth/login" : "/auth/register";
      const payload =
        mode === "login"
          ? { email: authForm.email, password: authForm.password }
          : {
              email: authForm.email,
              password: authForm.password,
              firstName: authForm.firstName,
              lastName: authForm.lastName
            };
      const data = await request(endpoint, {
        method: "POST",
        body: JSON.stringify(payload)
      });

      localStorage.setItem("token", data.token);
      localStorage.setItem("user", JSON.stringify(data.user));
      setCurrentUser(data.user);
      setAuthForm(emptyForm);
      setMessage(data.message);
      await loadUsers();
    } catch (error) {
      setMessage(error.message);
    }
  }

  async function handleUserSubmit(event) {
    event.preventDefault();
    setMessage("");

    try {
      if (editingId) {
        await request(`/users/${editingId}`, {
          method: "PUT",
          body: JSON.stringify({
            email: userForm.email,
            password: userForm.password || undefined,
            firstName: userForm.firstName,
            lastName: userForm.lastName,
            role: userForm.role
          })
        });
        setMessage("Utilisateur modifié.");
      } else {
        await request("/users", {
          method: "POST",
          body: JSON.stringify(userForm)
        });
        setMessage("Utilisateur créé.");
      }

      setUserForm(emptyForm);
      setEditingId(null);
      await loadUsers();
    } catch (error) {
      setMessage(error.message);
    }
  }

  function startEdit(user) {
    setEditingId(user.id);
    setUserForm({
      email: user.email,
      password: "",
      firstName: user.firstName,
      lastName: user.lastName,
      role: user.role
    });
  }

  async function deleteUser(id) {
    setMessage("");

    try {
      await request(`/users/${id}`, { method: "DELETE" });
      setMessage("Utilisateur supprimé.");
      await loadUsers();
    } catch (error) {
      setMessage(error.message);
    }
  }

  function logout() {
    localStorage.removeItem("token");
    localStorage.removeItem("user");
    setCurrentUser(null);
    setUsers([]);
    setEditingId(null);
    setUserForm(emptyForm);
    setMessage("Déconnecté.");
  }

  return (
    <main className="page">
      <section className="panel hero">
        <p className="eyebrow">Architecture 3 tiers</p>
        <h1>Authentification et CRUD utilisateurs</h1>
        <p className="lead">
          Frontend React, backend Express, PostgreSQL, Prisma, JWT, rate limiting
          et redondance backend via Nginx.
        </p>
        <div className="status">
          <span>{currentUser ? `Connecté: ${currentUser.email}` : "Non connecté"}</span>
          {currentUser ? <button onClick={logout}>Se déconnecter</button> : null}
        </div>
        {message ? <p className="message">{message}</p> : null}
      </section>

      {!currentUser ? (
        <section className="panel auth">
          <div className="tabs">
            <button
              className={mode === "login" ? "active" : ""}
              onClick={() => setMode("login")}
            >
              Connexion
            </button>
            <button
              className={mode === "register" ? "active" : ""}
              onClick={() => setMode("register")}
            >
              Inscription
            </button>
          </div>

          <form onSubmit={handleAuthSubmit}>
            <input
              placeholder="Email"
              type="email"
              value={authForm.email}
              onChange={(event) =>
                setAuthForm({ ...authForm, email: event.target.value })
              }
              required
            />
            <input
              placeholder="Mot de passe"
              type="password"
              value={authForm.password}
              onChange={(event) =>
                setAuthForm({ ...authForm, password: event.target.value })
              }
              required
            />
            {mode === "register" ? (
              <>
                <input
                  placeholder="Prénom"
                  value={authForm.firstName}
                  onChange={(event) =>
                    setAuthForm({ ...authForm, firstName: event.target.value })
                  }
                  required
                />
                <input
                  placeholder="Nom"
                  value={authForm.lastName}
                  onChange={(event) =>
                    setAuthForm({ ...authForm, lastName: event.target.value })
                  }
                  required
                />
              </>
            ) : null}
            <button type="submit">
              {mode === "login" ? "Se connecter" : "S'inscrire"}
            </button>
          </form>
        </section>
      ) : (
        <section className="dashboard">
          <section className="panel">
            <h2>{editingId ? "Modifier un utilisateur" : "Créer un utilisateur"}</h2>
            <form onSubmit={handleUserSubmit} className="user-form">
              <input
                placeholder="Email"
                type="email"
                value={userForm.email}
                onChange={(event) =>
                  setUserForm({ ...userForm, email: event.target.value })
                }
                required
              />
              <input
                placeholder={editingId ? "Nouveau mot de passe (optionnel)" : "Mot de passe"}
                type="password"
                value={userForm.password}
                onChange={(event) =>
                  setUserForm({ ...userForm, password: event.target.value })
                }
                required={!editingId}
              />
              <input
                placeholder="Prénom"
                value={userForm.firstName}
                onChange={(event) =>
                  setUserForm({ ...userForm, firstName: event.target.value })
                }
                required
              />
              <input
                placeholder="Nom"
                value={userForm.lastName}
                onChange={(event) =>
                  setUserForm({ ...userForm, lastName: event.target.value })
                }
                required
              />
              <select
                value={userForm.role}
                onChange={(event) =>
                  setUserForm({ ...userForm, role: event.target.value })
                }
              >
                <option value="user">user</option>
                <option value="admin">admin</option>
              </select>
              <div className="form-actions">
                <button type="submit">{editingId ? "Enregistrer" : "Ajouter"}</button>
                {editingId ? (
                  <button
                    type="button"
                    className="secondary"
                    onClick={() => {
                      setEditingId(null);
                      setUserForm(emptyForm);
                    }}
                  >
                    Annuler
                  </button>
                ) : null}
              </div>
            </form>
          </section>

          <section className="panel">
            <div className="list-header">
              <h2>Liste des utilisateurs</h2>
              <button className="secondary" onClick={loadUsers}>
                Rafraîchir
              </button>
            </div>
            <div className="table-wrap">
              <table>
                <thead>
                  <tr>
                    <th>ID</th>
                    <th>Email</th>
                    <th>Prénom</th>
                    <th>Nom</th>
                    <th>Rôle</th>
                    <th>Actions</th>
                  </tr>
                </thead>
                <tbody>
                  {users.map((user) => (
                    <tr key={user.id}>
                      <td>{user.id}</td>
                      <td>{user.email}</td>
                      <td>{user.firstName}</td>
                      <td>{user.lastName}</td>
                      <td>{user.role}</td>
                      <td className="actions">
                        <button className="secondary" onClick={() => startEdit(user)}>
                          Modifier
                        </button>
                        <button className="danger" onClick={() => deleteUser(user.id)}>
                          Supprimer
                        </button>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </section>
        </section>
      )}
    </main>
  );
}
