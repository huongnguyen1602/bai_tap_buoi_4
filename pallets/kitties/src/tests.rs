use crate::{mock::*, Error, Kitty, Gender, Pallet};
use frame_support::{assert_noop, assert_ok};


#[test]
fn check_gender(){
    let res1 = match Pallet::<Test>::gen_gender(&vec![0;16]){
        Ok(g) => g,
        _ => Gender::Female,
    };
    let res2 = match Pallet::<Test>::gen_gender(&vec![0;5]) {
        Ok(g) => g,
        _ => Gender::Female,        
    };
	assert_eq!(res1,Gender::Male);
    assert_eq!(res2,Gender::Female);

}

#[test]
fn check_create() {
	new_test_ext().execute_with(|| {
        // account, dna và gender
        let account = 1u64;
        let dna: Vec<u8> = vec![1,2,3,4];
        let gender = match KittyModule::gen_gender(&dna){
            Ok(g) => g,
            _ => Gender::Female,
        };
        // Lấy id kitty hiện tại
        let current_id = KittyModule::kitty_id();
        // Tạo kitty
        assert_ok!(KittyModule::create_kitty(RuntimeOrigin::signed(account.clone()), dna.clone()));
        // kiểm tra id hiện tại khi tạo thêm 1 kitty
        assert_eq!(current_id+1, KittyModule::kitty_id());
        // Kiểm tra kitty owned
        assert_eq!(KittyModule::kitty_owned(account.clone()), vec![dna.clone()]);
        // Kiểm tra kitty lưu trữ
        assert_eq!(KittyModule::get_kitty(dna.clone()), Some(Kitty{
            dna: dna.clone(),
            price: 0,
            gender: gender.clone(),
            owner: account.clone(),
        }));
	});
}


#[test]
fn check_tranfer() {
	new_test_ext().execute_with(|| {
        // from, to và dna
        let from = 1u64;
        let dna_of_from = vec![1,2,3,4];
        let to = 2u64;
        let vec:Vec<Vec<u8>> = Vec::new();
        // from tạo kitty
        assert_ok!(KittyModule::create_kitty(RuntimeOrigin::signed(from.clone()), dna_of_from.clone()));
        // tranfer kitty từ from sang from
        assert_noop!(KittyModule::transfer(RuntimeOrigin::signed(from.clone()), from.clone(), dna_of_from.clone()), Error::<Test>::TransferToSelf);
        // tranfer to sang from
        assert_noop!(KittyModule::transfer(RuntimeOrigin::signed(to.clone()), from.clone(), dna_of_from.clone()), Error::<Test>::NotOwner);
        // tranfer kitty từ from sang to
        assert_ok!(KittyModule::transfer(RuntimeOrigin::signed(from.clone()), to.clone(), dna_of_from.clone()));
        
        // kiểm tra kitty owned của from
        assert_eq!(KittyModule::kitty_owned(from.clone()),vec);
        // kiểm tra kitty owned của to
        assert_eq!(KittyModule::kitty_owned(to.clone()),vec![dna_of_from.clone()]);
	});
}

