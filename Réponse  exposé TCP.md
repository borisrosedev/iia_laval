Asymétrique :

1.Hôte A envoie un FIN (demande de fermeture).

2.Hôte B envoie un ACK (accuse réception). À ce stade, la connexion est à moitié fermée.

3.Hôte B finit d'envoyer ses données restantes, puis envoie son propre FIN.

4.Hôte A envoie un dernier ACK.

Symétrique : 

Si une application plante ou si un pare-feu décide de couper la ligne, un paquet RST est envoyé.