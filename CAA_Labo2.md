## CAA - Labo2



**Auteur:** Chris Barros Henriques



L'application est un coffre permettant de stocker des fichiers chiffrés.



### Architecture



#### Argon2

Pour la dérivation de clés afin de permettre le chiffrement des fichiers, j'ai choisi Argon2 (plus précisément Argo2id) afin d'avoir un bon compromis entre sécurité contre les CPU attacks et les side-channel attacks.



Le choix des constantes est une partie importante et j'ai appliqué la méthode enseignée dans le cours qui est:

```
1. Select the number of parallel threads. 
2. Fix the number of memory that can be used.
3. Based on application, decide on how long users can wait (website : 1s, desktop login : 5s, infrequent login : >20s) 
4. Raise complexity until it reached maximum allowed time
```



Je suis donc arrivé à:

**Degré de parallélisation:** 4 threads lancés simultanément

**Utilisation de la mémoire:** 2GB

**Taille de sel:** 128 bits

**Temps d'exécution:** 1s car considéré comme une web app



#### Authentification

Il était demandé d'effectuer une authentification basée sur un challenge-response. Voici donc le protocole:



- Le client envoie son nom d'utilisateur au serveur
- Si le serveur a cet utilisateur dans sa base données, il lui envoie un nonce ainsi qu'un sel pour permettre la dérivation de clé du MAC. Si le serveur ne reconnaît pas l'utilisateur, il envoie une erreur.
- Le client calcule le MAC réalisé avec la clé dérivée d'argon2 et le challenge envoyé par le serveur. Si les MAC ne correspondent pas, le server renvoie une erreur.
- Le serveur demande ensuite le token de Google Authenticator de l'utilisateur.  Si le token entré est correct, le client est finalement authentifié, sinon le serveur renvoie une erreur.



#### Chiffrement

Chaque fichier est chiffré via une clé dérivée produite à l'aide d'argon2 avec AES-GCM-SIV. À chaque fois que l'utilisateur upload un fichier dans le vault, le serveur ajoute dans sa base de données le nom chiffré du fichier, le sel qui a permis à argon2 de produire la clé dérivée et le nonce qu'a besoin AES-GCM-SIV pour le chiffrement. 

Pour le choix de la taille de clé, j'ai choisi de suivre les recommandations d'[ECRYPT](https://www.keylength.com/fr/3/) qui indique de prendre des clés de 256 bits pour les chiffrements symétriques.



Lorsque l'utilisateur choisit un fichier et veut lire le contenu du fichier, le serveur lui envoie alors le sel et le nonce correspondant.

Cette manière de faire permet à un utilisateur de ne nécessiter que d'un seul mot de passe pour déchiffrer tous les fichiers qu'il uploadera dans son vault, ce qui change c'est la clé dérivée. De plus, de cette façon, même si une clé qui permet le déchiffrement d'un des fichiers est leakée, celle-ci sera inutile pour le déchiffrement de tous les autres, ce qui importe étant le mot de passe master.