package com.phodal.pepper.refactor.staticclass;

class FileVerifyFactoryImpl implements FileVerifyFactory {
    @Override
    public FileVerify genFileVerify() {
        return new FileVerify();
    }
}
