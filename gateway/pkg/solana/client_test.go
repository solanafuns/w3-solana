package solana

import (
	"testing"

	"github.com/blocto/solana-go-sdk/common"
	"github.com/spf13/viper"
)

func TestConfigAccount(t *testing.T) {
	program := common.PublicKeyFromString("3P8bzeDY4i2QFU7AYtKH9jqnZqa7gShqB56tJPQDvmsS")
	t.Logf("program is %s", program.ToBase58())
	bytes := program.Bytes()
	t.Logf("bytes is -> %v ", bytes)

	name := "w3sol"

	baseSeed := ".w3-solana-name"
	seeds := [][]byte{}
	seeds = append(seeds, []byte(baseSeed))
	seeds = append(seeds, []byte(name))

	t.Logf("seeds is -> %v", seeds)

	k, bump, err := common.FindProgramAddress(seeds, program)
	t.Error(err)
	t.Logf("key is %s", k.ToBase58())
	t.Logf("bump is %d", bump)

}

func TestLoadPageAccount(t *testing.T) {
	viper.SetDefault("network", "localnet")
	program := common.PublicKeyFromString("3P8bzeDY4i2QFU7AYtKH9jqnZqa7gShqB56tJPQDvmsS")

	path := "/index.html"

	seeds := [][]byte{}
	seeds = append(seeds, []byte(path))

	k, bump, err := common.FindProgramAddress(seeds, program)
	t.Error(err)
	t.Logf("key is %s", k.ToBase58())
	t.Logf("bump is %d", bump)

	pageData, err := GetClient().LoadAccountContent(&k)
	t.Error(err)
	t.Log(string(pageData))

	d, err := ParsePageContent(pageData)
	t.Error(err)
	t.Log(string(d.RawData))

}
