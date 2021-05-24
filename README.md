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

**Degré de parallélisation:** 2 threads lancés simultanément. Cela est représenté dans le code par le paramètre `OPSLIMIT_INTERACTIVE`.

**Utilisation de la mémoire:** ~67MB représenté dans le code par le paramètre `OPSLIMIT_INTERACTIVE`. 

**Taille de sel:** 128 bits

**Temps d'exécution:** 1s environ car considéré comme une web app 



#### Authentification

Il était demandé d'effectuer une authentification basée sur un challenge-response. Voici donc le protocole:



- Le client envoie son nom d'utilisateur au serveur
- Si le serveur a cet utilisateur dans sa base données, il lui envoie un nonce ainsi qu'un sel pour permettre la dérivation de clé du MAC. Si le serveur ne reconnaît pas l'utilisateur, il envoie une erreur.
- Le client calcule le MAC réalisé avec la clé dérivée d'argon2 et le challenge envoyé par le serveur. Si les MAC ne correspondent pas, le server renvoie une erreur.
- Le serveur demande ensuite le token de Google Authenticator de l'utilisateur.  Si le token entré est correct, le client est finalement authentifié, sinon le serveur renvoie une erreur.



##### **Compte existant:** 

**username: **caa_labo2 

**Password:** MyPassword



#### Chiffrement

Chaque fichier est chiffré via une clé dérivée produite à l'aide d'argon2 avec AES-GCM-SIV. À chaque fois que l'utilisateur upload un fichier dans le vault, le serveur ajoute dans sa base de données le nom chiffré du fichier, le sel qui a permis à argon2 de produire la clé dérivée et le nonce qu'a besoin AES-GCM-SIV pour le chiffrement. 

Pour le choix de la taille de clé, j'ai choisi de suivre les recommandations d'[ECRYPT](https://www.keylength.com/fr/3/) qui indique de prendre des clés de 256 bits pour les chiffrements symétriques.



Lorsque l'utilisateur choisit un fichier et veut lire le contenu du fichier, le serveur lui envoie alors le sel et le nonce correspondant.

Cette manière de faire permet à un utilisateur de ne nécessiter que d'un seul mot de passe pour déchiffrer tous les fichiers qu'il uploadera dans son vault, ce qui change c'est la clé dérivée. De plus, de cette façon, même si une clé qui permet le déchiffrement d'un des fichiers est leakée, celle-ci sera inutile pour le déchiffrement de tous les autres, ce qui importe étant le mot de passe master.



### Disclaimer

Une partie dans `server.rs` qui peut paraître étrange au premier abord est le code suivant:

```rust
lazy_static! {

    static ref DB : HashMap<String, User> = {

        let salt = argon2id13::gen_salt();
        let mut k = secretbox::Key([0; secretbox::KEYBYTES]);
        let secretbox::Key(ref mut kb) = k;
        argon2id13::derive_key(kb, PASSWORD.as_bytes(), &salt, argon2id13::OPSLIMIT_INTERACTIVE, argon2id13::MEMLIMIT_INTERACTIVE).unwrap();

        let mut map = HashMap::new();
        map.insert(USERNAME.to_string(), User::new(USERNAME.to_string(), *kb, salt));

        map
    };
}
```

Cette partie a été faite pour simplifier la programmation étant donné qu'il n'y a pas d'authentification implémentée, car je pars du principe que l'utilisateur existe déjà dans la base de données du serveur.



### Pour tester

Créer des fichiers dans le dossier `/src/client/files` et commencer à utiliser le programme. Il y a toujours le fichier `3.txt` (avec un message sympahique dedans :)).



**Bonus réalisés:** Authentification 2 facteurs