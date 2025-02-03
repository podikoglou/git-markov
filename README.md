# git-markov

scripts for experimentation with markov chains

## 1. download (some) code

```
./dl.sh
```

## 2. compile the markov crate

```
cd markov
cargo b -r
```

## 3. feed a model

(2 is the order -- you may customize it)
```
./feed.sh '*.ts' models/typescript.bc 2
```

## 4. complete some input using a model

```
./complete.sh models/typescript.bc 'export default'
```
