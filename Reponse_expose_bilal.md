Q1 — Quel MSS sera retenu ?
Le MSS retenu c'est 1200 octets.
Pourquoi ? Simplement parce que chaque hôte ne peut pas envoyer plus grand que ce que l'autre peut recevoir. Le client annonce 1460, le serveur annonce 1200, donc on prend le plus petit des deux → 1200.
Et si on vérifie avec le MTU du lien routeur-serveur : 1280 - 20 (IP) - 20 (TCP) = 1240, donc 1200 passe sans problème.

Q2 — Le segment de 1300 octets sera-t-il fragmenté ?
Oui forcément, 1300 > 1200 donc il dépasse le MSS retenu. Il sera coupé en deux :

un premier morceau de 1200 octets
un deuxième de 100 octets


Q3 — Taille totale pour 1000 octets de données
On additionne tout :

1000 octets de données
+ 20 octets d'en-tête TCP
+ 20 octets d'en-tête IP

= 1040 octets au total

Q4 — Taille du premier paquet SYN avec l'option MSS
Le SYN ne transporte jamais de données donc :

20 octets IP
+ 20 octets TCP
+ 4 octets option MSS

= 44 octets au total